#include <bitcoin/bitcoin.hpp>
#include <string.h>
#include <iostream>
#include <boost/asio.hpp>

using namespace bc;
using namespace bc::wallet;
using namespace bc::machine;
using namespace bc::chain;

#define CLOSING_CHANNEL_FEES (10000)
#define DELAY (600)

data_chunk uint32_to_data_chunk(const uint32_t& value)
{
    data_chunk returnerValue; //32 бита
    for(int i=3; i>=0; i--)
    {
       returnerValue.push_back( value>>(8*i));
    }

    return returnerValue;

}
ec_compressed XOR(const ec_compressed& point1, const ec_compressed& point2) //побитовое
{
    ec_compressed ReturnerValue;
    for(int i=0; i< ec_compressed_size; i++)
    {
        ReturnerValue[i]=point1[i]^point2[i];
    }

    return ReturnerValue;
}

void send_transaction(const transaction& tx, boost::asio::ip::tcp::socket* socket)
{
    data_chunk SenderData=tx.to_data();
    uint64_t SizeData=SenderData.size();

    socket->send(boost::asio::buffer(&SizeData,sizeof(uint64_t)));

    for(int i=0; i<SizeData; i++)
    {
       socket->send( boost::asio::buffer(&SenderData[i],sizeof(char)) );
    }

}

transaction receive_transaction(boost::asio::ip::tcp::socket* socket)
{
    uint64_t SizeOfStr;
    while(socket->receive( boost::asio::buffer(&SizeOfStr, sizeof(uint64_t)) ) != sizeof(uint64_t)){}; //ждем пока клиент отправит первые данные, размер транзакции в символах в кодировке base16

    //получаем транзакцию(base16 код в строку)
    data_chunk TxData;
    uint8_t curByte;

    for(uint64_t i=0; i <SizeOfStr; i++)
    {
        socket->receive(boost::asio::buffer(&curByte, sizeof(uint8_t)));
        TxData.push_back(curByte);
    }
    //std::cout<<Tx_str<<std::endl;

    transaction tx;
    tx.from_data(TxData,true,true);

    return tx;
}


transaction create_opening_tx(const ec_public& ServerPublicKey, const ec_public& ClientPubKey, const ec_secret& UserSecret)
{

    payment_address ClientAddress=ClientPubKey.to_payment_address();
    transaction tx; //создаваемая транзакция
    tx.set_version(1u);

    //входы транзакции(в идеале лучше было бы, еслиб программа сама находила и строила входы, как например с помощью функции getUTXO, суть ее в том, чтобы обраться к серверу(в данном случае libbitcoin серверу) и отправить запрос на получение данных из блокчейна, из которые уже можно бы было получить UTXO для конкретного адреса,но сервер возвращает пустое множество UTXO, для любого адреса. долгое время не смогя разобраться с этой проблемой, было решено пока возложить на пользователя задачу найти и вписать свой UTXO)
    //выходы можно самостоятельно посмотреть на сайте
    //https://live.blockcypher.com/btc-testnet/tx/62408b1b14ce9eea82b73b543cfb0bdfc4ec118b9d50c07e6c6d75ba3c6a7b59/
    //тут необходимо ввести хэш транзакции в которой есть непотраченный выход
    std::string PrevTxHash_str;
    std::cout<<"\n write your prev UTXO hash in base16, once output from this will input for channel's creation's transaction:\n";
    std::cin>>PrevTxHash_str;
    hash_digest PrevTxHash;
    decode_hash(PrevTxHash, PrevTxHash_str);

    //тут нужно ввести индекс непотраченного выхода, индексация начинается с нуля, выходы например на сайте
    //https://live.blockcypher.com/btc-testnet/tx/62408b1b14ce9eea82b73b543cfb0bdfc4ec118b9d50c07e6c6d75ba3c6a7b59/
    //расположены сверху внизу в порядке увеличения их индекса
    uint32_t PrevTxIndex;
    std::cout<<"\n write index unspended output of your UTXO, its output will be input:\n";
    std::cin>>PrevTxIndex;

    //желательно реализовать поиска непотраченных выходов на определенную сумму автоматически,
    //то есть просить пользователя ввести только адрес и уже пробегая по блокчейну искать выходы
    //в предположение, что машина пользователя не является полной биткоин нодой,
    //нужно отправлять запрос серверу для нахождения непотраченных выходов.
    //но так как программа писалась для демонстрации работы Lighing канала, было решено поиск непотраченных выходов временно оставить на пользователе

    std::string WeightOfChannel_btc;
    uint64_t WeightOfChannel_satoshi;
    std::cout<<"\n write weight of channel in BTC:\n";
    std::cin>>WeightOfChannel_btc;
    decode_base10(WeightOfChannel_satoshi,WeightOfChannel_btc, btc_decimal_places);

    //сдача пользователю, то есть открывающая транзакция (opening_tx) имеет 2 выхода - первый, на счет с мультподписью размер отправляеных на него биткоинов равен ширине канала, второй - сдача пользователю, остаток средств которые он хочет вернуть на свой адрес, он наберет их сам, учитывая какую сумму он хочет потратить на fees
    std::string OddMoney_btc;
    uint64_t OddMoney_satoshi;
    std::cout<<"\n write odd money (in BTC), for creating p2pkh output on your address. Dont forgot about transaction's fees:\n";
    std::cin>>OddMoney_btc;
    decode_base10(OddMoney_satoshi,OddMoney_btc, btc_decimal_places);


    output_point UTXO(PrevTxHash, PrevTxIndex);

    input input0;
    input0.set_previous_output(UTXO);
    input0.set_sequence(0xffffffff);

    tx.inputs().push_back(input0); //добавим вход без подписи

    //создадим выходы транзакции (2 выхода:
    //первый - на адрес с мультиподписью 2из2)
    //второй - сдача самому себе, выход со скриптом вида p2pkh

    point_list PubKeys({ServerPublicKey.point(), ClientPubKey.point()});
    operation::list MultisigLockingScript=script::to_pay_multisig_pattern(2u,PubKeys); //выход с мультиподписью
    /*
    MultisigLockingScript.push_back(operation(opcode::push_positive_1));
    MultisigLockingScript.push_back(operation(to_chunk(ServerPublicKey.point() )));
    MultisigLockingScript.push_back(operation(to_chunk(ClientPubKey.point() )));
    MultisigLockingScript.push_back(operation(opcode::push_positive_2));
    MultisigLockingScript.push_back(operation(opcode::checkmultisig));
    */
    output Output0(WeightOfChannel_satoshi,MultisigLockingScript); //вызываем конструктор класса output, в который передаем 1 - количество коинов, 2 - скрипт


    operation::list p2pkhScript=script::to_pay_key_hash_pattern(ClientAddress.hash()); //скрипт для возвращает сдачу себе
    output Output1(OddMoney_satoshi, p2pkhScript); //создаем второй выход

    tx.outputs().push_back(Output0);
    tx.outputs().push_back(Output1);

    //после добавления всех входов и выходов подпишем транзакцию

    endorsement Sig0;
   // считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
    script::create_endorsement(Sig0,UserSecret,p2pkhScript,tx,0,0x01);

    operation::list  sig_script0;
    sig_script0.push_back(operation(Sig0));
    sig_script0.push_back(operation(to_chunk(ClientPubKey.point() )));
    script InputScript0(sig_script0);
    tx.inputs()[0].set_script(InputScript0);

    return tx;
}

transaction create_commitment_tx(const transaction& funding_tx,const ec_public& ServerPublicKey, const ec_secret& UserSecret,
                                 const ec_compressed& revocation_basepoint, const ec_compressed& remote_per_commitment_basepoint,
                                 const uint64_t& MyBalance, const uint64_t& ServerBalance)
{

    ec_private UserPrivate(UserSecret, ec_private::testnet);
    ec_public UserPubKey=UserPrivate.to_public();

    transaction CommitTX;
    CommitTX.set_version(2u);

    //вычислим nSequence(необходим для задержки валюты), как описано на:
    //https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki
    uint32_t nTime=DELAY; //неделя в секундах
    uint32_t nSequence=(1<<22) | (nTime>>9);

    //вход
    output_point UTXO(funding_tx.hash(), 0u); //вход транзакции обязательства, это первый(с нулевым индексом) выход созданной ранее funding транзакции

    input input0;
    input0.set_previous_output(UTXO);
    input0.set_sequence(0xffffffff);

    CommitTX.inputs().push_back(input0); //добавим вход без подписи

    //выходы
    output output0;  //выход на свой адрес с p2pkh выходом
    output0.set_value(MyBalance - (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+ServerBalance)));
    operation::list p2pkhScript=script::to_pay_key_hash_pattern(UserPubKey.to_payment_address().hash());
    output0.set_script(p2pkhScript);

    //RSMC выход, выход со следуюшим скриптом:
    /*

OP_IF
    # Penalty transaction
    <revocationpubkey>
OP_ELSE
    `to_self_delay`
    OP_CSV
    OP_DROP
    <local_delayedpubkey>
OP_ENDIF
OP_CHECKSIG

более подробно можно почитать:
https://github.com/lightningnetwork/lightning-rfc/blob/master/03-transactions.md
разделы KEYS, revocationpubkey Derivation

составим revocationpubkey по формуле
revocationpubkey = revocation_basepoint * SHA256(revocation_basepoint || per_commitment_point) + per_commitment_point * SHA256(per_commitment_point || revocation_basepoint)
 , где || операция XOR(исключающее или)
 */
    ec_compressed revocation_pub_key;
    point_list commitment_point_list = {remote_per_commitment_basepoint,revocation_basepoint};
    //revocation_pub_key= remote_per_commitment_basepoint + revocation_basepoint. в поле эллиптических точек.
    ec_sum(revocation_pub_key, commitment_point_list);
    ec_multiply( revocation_pub_key, to_array<ec_secret_size>( sha256_hash_chunk( XOR(revocation_basepoint, remote_per_commitment_basepoint) )) );


    output output1;
    output1.set_value(ServerBalance- (ServerBalance * CLOSING_CHANNEL_FEES/(MyBalance+ServerBalance)));

    operation::list outputScript1;
    outputScript1.push_back(operation(opcode::if_));
    outputScript1.push_back(operation(to_chunk(revocation_pub_key)));
    outputScript1.push_back(operation(opcode::else_));
    //добавить to_self_delay
    outputScript1.push_back(operation(uint32_to_data_chunk(nSequence)));

    outputScript1.push_back(operation(opcode::checksequenceverify));
    outputScript1.push_back(operation(opcode::drop));
    outputScript1.push_back(operation(to_chunk(ServerPublicKey.point())));
    outputScript1.push_back(operation(opcode::endif));
    outputScript1.push_back(operation(opcode::checksig));

    output1.set_script(outputScript1);

    CommitTX.outputs().push_back(output0);
    CommitTX.outputs().push_back(output1);
    //подпишем транзакцию своим приватным ключом

    endorsement Sig0;
   // считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
    script::create_endorsement(Sig0,UserSecret,funding_tx.outputs()[0].script(),  CommitTX,0,0x01);

    operation::list  sig_script0;
    sig_script0.push_back(operation(Sig0));
    script InputScript0(sig_script0);
    CommitTX.inputs()[0].set_script(InputScript0);

    return CommitTX;
}

void client_handler(boost::asio::ip::tcp::socket* socket ,const boost::system::error_code & ec)
{
    if( ec)
    {
        std::cout<<ec.message()<<std::endl;
        return;
    }

    ec_public  ServerPublicKey("033ddf60c1191d4e16e4f59a6fa3b5e899c47915a634460d2add2b6bdec0b4c3c6"); //адрес сервера нужно по идее передавать по сети

    std::string ClientPubKey_str;
    std::cout<<"enter your public key in base16:\n";
    std::cin>>ClientPubKey_str;
    ec_public ClientPubKey(ClientPubKey_str);

    //для подписи требуется приватный ключ пользователя
    std::string UserSecret_str;
    std::cout<<"write your private key in base16 (it need for sifnaturing of tx, and doesnt be send on server):\n";
    std::cin>>UserSecret_str;
    ec_secret UserSecret;
    decode_base16(UserSecret, UserSecret_str);
    ec_private UserPrivate(UserSecret, ec_private::testnet);
    if(UserPrivate.to_public() != ClientPubKey) {std::cout<<"error: Invalid private key for inputed before public key\n";}



    transaction funding_tx=create_opening_tx(ServerPublicKey, ClientPubKey, UserSecret);


    //cout<<"\n\n tx: "<<encode_base16(tx.to_data(true,true)) << std::endl;

    //отправляем транзакцию серверу для проверки на наличие выхода с мультиподписью и для трансляции в блокчейн
    send_transaction(funding_tx,socket);
    std::cout<<"fundibg tx hash: \n"<< encode_base16( funding_tx.hash() )<<std::endl;

    //ждем ответ от сервера
    bool returnerFlag;
    while(socket->receive( boost::asio::buffer(&returnerFlag, sizeof(bool)) ) != sizeof(bool)){}; //ждем пока клиент отправит первые данные, размер транзакции в символах в кодировке base16

    if(!returnerFlag){std::cout<<"Error: Transaction is not valid.\n"; return;}
    std::cout<<"Transaction broadcasted in Blockchain, now creating commit-transaction\n\n";

    //генерируем секретное значение
    //как описано на:
    //https://github.com/lightningnetwork/lightning-rfc/blob/master/03-transactions.md
    //для начала генерируем значение revocation_basepoint_secret

    // The secp256k1 Generator Point.(базовая точка биткоина)
    auto gen_point = base16_literal(
        "0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");

    data_chunk my_entropy(ec_secret_size); //256bits
    pseudo_random_fill(my_entropy);

    // Instantiate private key with 256 bits of entropy.
    ec_secret revocation_basepoint_secret = to_array<ec_secret_size>(my_entropy);
    ec_compressed revocation_basepoint(gen_point);

    ec_multiply(revocation_basepoint, revocation_basepoint_secret);




    //генерируем per_commitment_basepoint, и соответсвенно per_commitment_secret
    pseudo_random_fill(my_entropy);
    ec_secret per_commitment_secret = to_array<ec_secret_size>(my_entropy);
    ec_compressed per_commitment_basepoint(gen_point);
    ec_multiply(per_commitment_basepoint, per_commitment_secret);

    //отправляем revocation_basepoint серверу
    socket->send(boost::asio::buffer(&revocation_basepoint, ec_compressed_size));

    ec_compressed remote_revocation_basepoint;
    //получаем от сервера remote_revocation_basepoint
    while(socket->receive(boost::asio::buffer(&remote_revocation_basepoint, ec_compressed_size)) != ec_compressed_size){}


    //отправляем per_commitment_basepoint серверу
    socket->send(boost::asio::buffer(&per_commitment_basepoint, ec_compressed_size));
    ec_compressed remote_per_commitment_basepoint;


    //получаем от сервера remote_per_commitment_basepoint
    while(socket->receive(boost::asio::buffer(&remote_per_commitment_basepoint, ec_compressed_size)) != ec_compressed_size){}

    //внутри-канальные балансы
    uint64_t MyBalance=funding_tx.outputs()[0].value()-1000u;
    uint64_t ServerBalance=1000u;
    //создаем транзакцию-обязательство

    //создаем первую транзакцию обязательство
    transaction commit_tx=create_commitment_tx(funding_tx,ServerPublicKey, UserSecret, revocation_basepoint, remote_per_commitment_basepoint,MyBalance,ServerBalance);

    //отправляем серверу первую транзакцию-обязательство
    send_transaction(commit_tx,socket);

    //получаем от сервера его транзакцию
    transaction remote_commit_tx=receive_transaction(socket);

    //подпишем полченную транзакцию обязательство
    endorsement Sig0;
    operation::list sig_script0;
    script::create_endorsement(Sig0,UserSecret, funding_tx.outputs()[0].script(),  remote_commit_tx,0,0x01);

    sig_script0.push_back(operation(opcode::push_size_0));
    sig_script0.push_back(remote_commit_tx.inputs()[0].script().operations()[0]);
    sig_script0.push_back(operation(Sig0));
    script InputScript0(sig_script0);
    remote_commit_tx.inputs()[0].set_script(InputScript0);



    std::cout<<"my coomit tx:\n"<<encode_base16(commit_tx.to_data(true,true))<<std::endl;

    std::cout<<"server coomit tx:\n"<<encode_base16(remote_commit_tx.to_data(true,true))<<std::endl;


    std::cout<<"channel was created,";    //обмениваемся с сервером сообщениями
    std::string Command;
    int8_t message=0;
    uint64_t sum;
    while(message!=1)
    {
        std::cout<<" write \"-pay\" for payment by channel, or \"-close\" for closing of channel\n";
        std::cout<<"your in-channel Balance: "<< encode_base10(MyBalance, btc_decimal_places)<<std::endl;
        std::cout<<"servet in-channel Balance: "<<encode_base10(ServerBalance, btc_decimal_places)<<std::endl;
        std::cin>>Command;
        if(Command=="-pay") //хотим отправить серверу плату
        {
            message=0;
            std::cout<<"write summ of pay:\n";
            std::cin>>Command;
            decode_base10(sum,Command, btc_decimal_places);

            while(sum>MyBalance || sum==0)
            {
                std::cout<<"your in-channel balance is "<<MyBalance<<"please write summ less that it value:";
                std::cin>>Command;
                decode_base10(sum,Command, btc_decimal_places);
            }

            //дадим серверу знать о том, что мы хотим совершить платеж по каналу.
            socket->send(boost::asio::buffer(&message, sizeof(int8_t)));
            //передадим сумму которую желаем передать по каналу
            socket->send(boost::asio::buffer(&sum, sizeof(uint64_t)));

            //меняем балансы
            MyBalance-=sum;
            ServerBalance+=sum;
            //отправим серверу секретное значение(per_commitment_secret), для старой транзакции
            socket->send(boost::asio::buffer(&per_commitment_secret, sizeof(ec_secret)));
            //генерируем новые per_commitment_secret и per_commitment_basepoint
            pseudo_random_fill(my_entropy);
            per_commitment_secret = to_array<ec_secret_size>(my_entropy);
            per_commitment_basepoint=gen_point;
            ec_multiply(per_commitment_basepoint, per_commitment_secret);
            //отправляем сгенерированную только что per_commitment_basepoint серверу
            socket->send(boost::asio::buffer(&per_commitment_basepoint, ec_compressed_size));

            //отправим серверу хэш от предыдущей транзакции обязательстве(которую отправлял сервер, а клинт подписал после предыдуще оплаты)
            hash_digest cur_hash=remote_commit_tx.hash();
            socket->send(boost::asio::buffer(&cur_hash, hash_size));
            //получаем от сервера его транзакцию обязательство (с обновленным балансом)
            transaction current_remote_commit_TX=receive_transaction(socket);
            //std::cout<<"commit tx code for sending in blockchain:\n"<<encode_base16( current_remote_commit_TX.hash() );


            //подпишем полченную транзакцию обязательство
            endorsement Sig0;
            operation::list sig_script0;
            script::create_endorsement(Sig0,UserSecret, funding_tx.outputs()[0].script(),  current_remote_commit_TX,0,0x01);

            sig_script0.push_back(operation(opcode::push_size_0));
            sig_script0.push_back(current_remote_commit_TX.inputs()[0].script().operations()[0]);
            sig_script0.push_back(operation(Sig0));

            script InputScript0(sig_script0);
            current_remote_commit_TX.inputs()[0].set_script(InputScript0);

            //проверяем полученную транзакцию обязательство(соответсвие входа выходу образующей транзакции канала, и выходных балансов. а также то, что для выходов клиент действительно использовал переданную ему от сервера отдоразовую точку)
            //оставим проверку на потои, пока представим идеальную ситуацию в которой клиент не пытается обмануть сервер




            //в случае удачных проверок(если транзакция-обязательство полученная от сервера валидна) заменяем
            remote_commit_tx=current_remote_commit_TX;
            std::cout<<"commit tx code for sending in blockchain:\n"<<encode_base16( remote_commit_tx.to_data() );
            std::cout<<"commit tx code for sending in blockchain:\n"<<encode_base16( remote_commit_tx.hash() );


            //отправляем серверу свою новую транзакцию обязательство
            commit_tx=create_commitment_tx(funding_tx,ServerPublicKey,UserSecret,revocation_basepoint,remote_per_commitment_basepoint,MyBalance,ServerBalance);
            send_transaction(commit_tx,socket);

            //возможно нужно подождать от сервера ответ - принимает ли он платеж или нет



        }
        if(Command=="-close") //просим сервер закрыть канал
        {
            message=1;
            //дадим серверу знать о том, что мы хотим закрыть канал.
            socket->send(boost::asio::buffer(&message, sizeof(int8_t)));
            //закрывать канал будем по алгоритму описанному на
            //https://lightning.network/lightning-network-paper.pdf
            //стр 28
            //для начала создаем транзакцию
            transaction lastTX;

            //вход
            output_point UTXO(funding_tx.hash(), 0u); //вход транзакции, это первый(с нулевым индексом) выход образующей канал транзакции

            input input0;
            input0.set_previous_output(UTXO);
            input0.set_sequence(0xffffffff);

            lastTX.inputs().push_back(input0); //добавим вход без подписи

            //выходы
            output output0;  //выход на свой адрес с p2pkh выходом
            output0.set_value(MyBalance- (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+ServerBalance)));
            operation::list p2pkhScript=script::to_pay_key_hash_pattern(ClientPubKey.to_payment_address().hash());
            output0.set_script(p2pkhScript);

            output output1;
            output1.set_value(ServerBalance- (ServerBalance * CLOSING_CHANNEL_FEES/(MyBalance+ServerBalance)));
            operation::list p2pkhScriptToServer=script::to_pay_key_hash_pattern(ServerPublicKey.to_payment_address().hash());
            output1.set_script(p2pkhScriptToServer);

            lastTX.outputs().push_back(output0);
            lastTX.outputs().push_back(output1);
            //подпишем транзакцию своим приватным ключом

            endorsement Sig0;
           // считаем скрипт выхода соответствующнго входу0
            script::create_endorsement(Sig0,UserSecret, funding_tx.outputs()[0].script(),  lastTX,0u,0x01);

            operation::list  sig_script0;
            //sig_script0.push_back(operation(opcode::push_size_0));
            sig_script0.push_back(operation(Sig0));
            script InputScript0(sig_script0);
            lastTX.inputs()[0].set_script(InputScript0);

            //после подписания отправим транзакцию на сервер
            send_transaction(lastTX,socket);

            /*далее мы считаем канал закрытым, так как сервер получив такое сообщение в любом случае закроет канал,
              делать он это будет по следующему алгоритму:
              1)проверит полученную последнюю транзакцию клиента(адрес входа, выходы, соответствие балансов адресам)
                если транзакция прошла проверку успешно:
                1.1) сервер подписывает ее своим приватным ключом и отправляет в блокчейн
                если транзакция не прошла проверку:
                1.2) сервер подпишет последнюю полученную от клиента транзакцию обязательство и отправит ее в блокчейн(клиент по любому получает свои средства)

           Данный подход (да и программа в целом) сырой так как не учитывает возможно разрыва соединения между клиентом и сервером.
           То есть программа корректно создает, передает средства и закрывает канал сеансово(во время одного сеанса), что подходит только для тестирования или демонстрации работы алгоритма.
           Для того же чтобы программу можно было полноценно эксплуатировать ее следует еще доработать, а именно:

            1)учесть возможность разрыва соединения, и либо добавить функционал для переподключения к серверу с существующим каналом, и закрывать канал при потере соединения(например пользователь не переподключается какое-то время)
            2)учитывать возможные ошибки (искажения или инвертирования некоторых бит под воздействием шумов) при передаче строки:
                2.1) либо при каждой передаче данных передавать вместе с данными их хэш, а при получение проверять соответствуют ли данные хэшу и если нет, то запросить повторную отправку.
                2.2) или же, что еще более надежно, при открытие канала каждая из стороны генерирует по паре откр./закр. ключей, обменивается закрытыми и далее все свои сообщения подписывает своим закрытым ключом.
                Таким образом можно быть уверенным не только в том, что были получены не искаженные данные, но еще и в том, что они отправленным именно тем пользователем с которым и был открыт канал.


            3) на сервере вывести проверку на то входит ли каждая предыдущая транзакция обязательство пользователя в сеть или нет в отдельны поток, на данны момент все выполняется в одном потоке, и сервер после каждо команды пользователя делает такую проверку, что существенно замедляет отклик сервера на команды клиента.

            4)на данный момент сервер сам можно сказать является клиентом, он не является биткоин нодой, а для проверок валидности транзакции и наличие
            */





        }
    }










}

int main()
{
    std::string Command;

    boost::asio::io_service IOservice;
   boost::asio::ip::tcp::endpoint Endpoint(boost::asio::ip::address_v4::from_string("127.0.0.1"), 17666); //localhost, порт 17666
   boost::asio::ip::tcp::socket* Socket=new boost::asio::ip::tcp::socket(IOservice);

    while(true)
    {
        std::cout<<"write \"-create_channel\" for creating channel or \"-exit\"\n";
        std::cin>>Command;
        if(Command=="-exit") {break;}
        if(Command=="-create_channel")
        {

            Socket->async_connect(Endpoint, std::bind( client_handler, Socket, std::placeholders::_1) );
            IOservice.run();
        }
    }


}
