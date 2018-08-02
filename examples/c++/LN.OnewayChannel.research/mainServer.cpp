#include <bitcoin/bitcoin.hpp>
#include <bitcoin/client.hpp>
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



ec_compressed XOR(const ec_compressed& point1, const ec_compressed& point2) //побитовое
{
    ec_compressed ReturnerValue;
    for(int i=0; i< ec_compressed_size; i++)
    {
        ReturnerValue[i]=point1[i]^point2[i];
    }

    return ReturnerValue;
}

int is_it_tx_in_blockchain(const hash_digest& tx_hash)
{
    client::connection_type connection = {};
    connection.retries = 3;
    connection.timeout_seconds = 8;
    connection.server = config::endpoint("tcp://testnet1.libbitcoin.net:19091");
    client::obelisk_client client(connection);

    int val1=0;
    static const auto on_done = [&val1](transaction tx) {
        val1=1;

    };

    static const auto on_error = [&val1](const code& ec) {

    };

    if(!client.connect(connection))
    {
        val1=-1; //если функция возвращает -1, это значит, что нету соединения с сервером libbitcoin.
    }

    client.blockchain_fetch_transaction2(on_error, on_done,tx_hash);
    client.transaction_pool_fetch_transaction2(on_error,on_done,tx_hash);
    client.wait();


    //return allPoints;
    return val1;


}

int check_all_prev_tx(std::vector<hash_digest>& prevs_tx)
{

    std::cout<<prevs_tx.size()<<std::endl;
    int curInd=0;
    for(std::vector<hash_digest>::iterator it = prevs_tx.begin(); it!=prevs_tx.end(); it++)
    {
        hash_digest cur_tx = (*it);
        std::cout<<encode_base16( cur_tx )<<std::endl;
        int value_for_cur_tx=is_it_tx_in_blockchain( cur_tx);
        if( value_for_cur_tx==1)
        {
           return curInd; //возвращаем индекс транзакции которая обнаружена в блокчейне
        }
        if(value_for_cur_tx==-1)
        {
            return -2; //проблемы с соединением
        }
        curInd++;
    }

    return -1; //возвращаем -1, значит в блокчейне не найдено не одной транзакции из перечисленных
}

bool validate_tx(const transaction& tx)
{
    client::connection_type connection = {};
    connection.retries = 3;
    connection.timeout_seconds = 8;
    connection.server = config::endpoint("tcp://testnet1.libbitcoin.net:19091");
    client::obelisk_client client(connection);

    bool val1;
    static const auto on_done = [&val1](const std::error_code& ec) {
        std::cout<<" Transaction validate:"<<ec.message()<<std::endl;

        if(ec) val1=false;
        else val1=true;

    };

    static const auto on_error = [](const code& ec) {

        std::cout << "Error Code: " << ec.message() << std::endl;

    };

    if(!client.connect(connection))
    {
        std::cout << "Fail" << std::endl;
    } else {
        std::cout << "Connection Succeeded" << std::endl;
    }

    client.transaction_pool_validate2(on_error,on_done,tx);
    client.wait();


    //return allPoints;
    return val1;


}

bool broadcast_tx(const transaction& tx)
{
    client::connection_type connection = {};
    connection.retries = 3;
    connection.timeout_seconds = 8;
    connection.server = config::endpoint("tcp://testnet1.libbitcoin.net:19091");
    client::obelisk_client client(connection);

    bool val1;
    static const auto on_done = [&val1](const std::error_code& ec) {
        std::cout<<" Transaction validate:"<<ec.message()<<std::endl;

        if(ec) val1=false;
        else val1=true;

    };

    static const auto on_error = [](const code& ec) {

        std::cout << "Error Code: " << ec.message() << std::endl;

    };

    if(!client.connect(connection))
    {
        std::cout << "Fail" << std::endl;
    } else {
        std::cout << "Connection Succeeded" << std::endl;
    }

    client.transaction_pool_broadcast(on_error, on_done, tx);

    client.wait();


    //return allPoints;
    return val1;


}

transaction create_after_timelock_transaction(const transaction& broadcasted_commitment_tx, const ec_public& MyPublicKey, const ec_secret& MySecret) //после того как отправили транзакцию-обязательство в сеть, вторая сторона сразу получает коины, а отправитель ждем 7 дней, создаем транзакцию для вывода средства через семь дней
{
    transaction timeunlock_tx;
    timeunlock_tx.set_version(2u);

    //вычислим nSequence(необходим для задержки валюты), как описано на:
    //https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki
    uint32_t nTime=DELAY; //неделя в секундах
    uint32_t nSequence=(1<<22) | (nTime>>9);

    //вход
    output_point RSMC_Out(broadcasted_commitment_tx.hash(), 1u); //вход транзакции обязательства, это первый(с нулевым индексом) выход созданной ранее транзакции

    input input0;
    input0.set_previous_output(RSMC_Out);
    input0.set_sequence(nSequence);
    operation::list inputScript;
    inputScript.push_back(operation(opcode::push_size_0));
    input0.set_script(inputScript);
    timeunlock_tx.inputs().push_back(input0); //добавим вход без подписи

    //выходы
    output output0;  //выход на свой адрес с p2pkh выходом
    output0.set_value(broadcasted_commitment_tx.outputs()[1u].value() - CLOSING_CHANNEL_FEES);
    operation::list p2pkhScript=script::to_pay_key_hash_pattern(MyPublicKey.to_payment_address().hash());
    output0.set_script(p2pkhScript);
    timeunlock_tx.outputs().push_back(output0);

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
 */

    //подпишем транзакцию
    endorsement Sig0;
   // считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
    script::create_endorsement(Sig0, MySecret, broadcasted_commitment_tx.outputs()[1u].script(),  timeunlock_tx,0,0x01);

    operation::list  sig_script0;
    sig_script0.push_back(operation(Sig0));
    sig_script0.push_back(operation(opcode::push_size_0));
    script InputScript0(sig_script0);
    timeunlock_tx.inputs()[0].set_script(InputScript0);

    return timeunlock_tx;
}

transaction create_commitment_tx(const transaction& opening_tx,const ec_public& EitherPublicKey, const ec_secret& UserSecret,
                                 const ec_compressed& revocation_basepoint, const ec_compressed& remote_per_commitment_basepoint,
                                 const uint64_t& MyBalance, const uint64_t& EitherBalance)
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
    output_point UTXO(opening_tx.hash(), 0u); //вход транзакции обязательства, это первый(с нулевым индексом) выход созданной ранее транзакции

    input input0;
    input0.set_previous_output(UTXO);
    input0.set_sequence(0xffffffff);

    CommitTX.inputs().push_back(input0); //добавим вход без подписи

    //выходы
    output output0;  //выход на свой адрес с p2pkh выходом
    output0.set_value(MyBalance- (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+EitherBalance)));
    operation::list p2pkhScript=script::to_pay_key_hash_pattern(UserPubKey.to_payment_address().hash());
    output0.set_script(p2pkhScript);
    CommitTX.outputs().push_back(output0);

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
    output1.set_value(EitherBalance- (EitherBalance * CLOSING_CHANNEL_FEES/(MyBalance+EitherBalance)));

    operation::list outputScript1;
    outputScript1.push_back(operation(opcode::if_));
    outputScript1.push_back(operation(to_chunk(revocation_pub_key)));
    outputScript1.push_back(operation(opcode::else_));
    //добавить to_self_delay
    outputScript1.push_back(operation(uint32_to_data_chunk(nSequence)));

    outputScript1.push_back(operation(opcode::checksequenceverify));
    outputScript1.push_back(operation(opcode::drop));
    outputScript1.push_back(operation(to_chunk(EitherPublicKey.point())));
    outputScript1.push_back(operation(opcode::endif));
    outputScript1.push_back(operation(opcode::checksig));

    output1.set_script(outputScript1);

    CommitTX.outputs().push_back(output1);
    //подпишем транзакцию своим приватным ключом

    endorsement Sig0;
   // считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
    script::create_endorsement(Sig0,UserSecret,opening_tx.outputs()[0].script(),  CommitTX,0,0x01);

    operation::list  sig_script0;
    sig_script0.push_back(operation(Sig0));
    script InputScript0(sig_script0);
    CommitTX.inputs()[0].set_script(InputScript0);

    return CommitTX;
}




void server_handler(boost::asio::ip::tcp::socket* socket ,const boost::system::error_code & ec)
{
    ec_public ServerPublicKey("033ddf60c1191d4e16e4f59a6fa3b5e899c47915a634460d2add2b6bdec0b4c3c6"); //адрес сервера нужно передавать по сети
    ec_secret ServerSecret=base16_literal("89a89627de29087218d73d553572d7c1cc07da730bedeb8ff6b7a38059fb261b");

//полкчаем транзакцию(октрывающую канал)
transaction tx=receive_transaction(socket);

//cout<<"tx: "<<encode_base16(tx.to_data(true,true)) << std::endl;

//проверим, чтобы в переданной транзакции был выход с мультиподписью
//этот выходи всегда имеет индекс 0, если транзакция была составлена по алгоритму клиента
ec_public ClientPublicKey;
operation::list OutWithMultisig=tx.outputs()[0].script().operations();
if(OutWithMultisig.size()==5 &&
   OutWithMultisig[0]==operation(opcode::push_positive_2) &&
   OutWithMultisig[1]==operation(to_chunk(ServerPublicKey.point() )) &&
   OutWithMultisig[3]==operation(opcode::push_positive_2) &&
   OutWithMultisig[4]==operation(opcode::checkmultisig)
   )
{
    //получаем публичный ключ клиента
    ClientPublicKey= encode_base16( OutWithMultisig[2].to_data() ).substr(2);
    std::cout<<"Client's Puiblic key is:\n"<< ClientPublicKey.encoded()<<std::endl;
}

bool ResultFlag;
//проверим транзакцию на корректность
if( !validate_tx(tx) )
{
    //отправить клиенту сигнал о фейле, этот сигнал
    ResultFlag=false;
    socket->send(boost::asio::buffer( &ResultFlag, sizeof(bool)) );
     return;
}

//транслируем транзакцию в сеть
if( !broadcast_tx(tx) )
{
    //отправить клиенту сигнал о фейле, этот сигнал
    ResultFlag=false;
    socket->send(boost::asio::buffer( &ResultFlag, sizeof(bool)) );
     return;
}
//отправить клиенту, сигнал о том, что транзакция проверена сервером и отправлена в сеть блокчейна
ResultFlag=true;
socket->send(boost::asio::buffer( &ResultFlag, sizeof(bool)) );

//первым делом после отправки открывающей транзакции, генерируем  необходимые для создания транзакции обязательства переменные
//но сперва балансы канала
uint64_t ClientBalance=tx.outputs()[0].value()-1000u;
uint64_t MyBalance=1000u;


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

//получаем от клиента remote_revocation_basepoint
ec_compressed remote_revocation_basepoint;
while(socket->receive(boost::asio::buffer(&remote_revocation_basepoint, ec_compressed_size)) != ec_compressed_size){}

//отправляем revocation_basepoint клиенту
socket->send(boost::asio::buffer(&revocation_basepoint, ec_compressed_size));

//получаем от клиента remote_per_commitment_basepoint
ec_compressed remote_per_commitment_basepoint;
while(socket->receive(boost::asio::buffer(&remote_per_commitment_basepoint, ec_compressed_size)) != ec_compressed_size){}

//отправляем per_commitment_basepoint клиенту
socket->send(boost::asio::buffer(&per_commitment_basepoint, ec_compressed_size));

//cоздаем первую транзакцию обязательство
transaction commitTX=create_commitment_tx(tx,ClientPublicKey,ServerSecret,revocation_basepoint,remote_per_commitment_basepoint,MyBalance,ClientBalance);

//получаем первую транзакцию обязательство(base16 код в строку)
transaction remote_commit_tx=receive_transaction(socket);

//отправляем клиенту первую транзакцию обязательство
send_transaction(commitTX,socket);

//проверяем полученную транзакцию обязательство(соответсвие входо, и выходных балансов)
//для начала подпишем ее
endorsement Sig0;
operation::list sig_script0;
script::create_endorsement(Sig0,ServerSecret, tx.outputs()[0].script(),  remote_commit_tx,0,0x01);

sig_script0.push_back(operation(opcode::push_size_0));
sig_script0.push_back(operation(Sig0));
sig_script0.push_back(remote_commit_tx.inputs()[0].script().operations()[0]);
script InputScript0(sig_script0);
remote_commit_tx.inputs()[0].set_script(InputScript0);

//создадим скрипт, такой же, какой должен был создать клиент для RSMC выхода, создадим его для сравнения его со скриптом полученным в подписанной пользователем транзакции.
ec_compressed remote_revocation_pub_key;
point_list commitment_point_list = {per_commitment_basepoint,remote_revocation_basepoint};
//revocation_pub_key= remote_per_commitment_basepoint + revocation_basepoint. в поле эллиптических точек.
ec_sum(remote_revocation_pub_key, commitment_point_list);
ec_multiply( remote_revocation_pub_key, to_array<ec_secret_size>( sha256_hash_chunk( XOR(remote_revocation_basepoint, per_commitment_basepoint) )) );

operation::list RSMCScript;
RSMCScript.push_back(operation(opcode::if_));
RSMCScript.push_back(operation(to_chunk(remote_revocation_pub_key)));
RSMCScript.push_back(operation(opcode::else_));
//добавить to_self_delay
//вычислим nSequence(необходим для задержки валюты), как описано на:
//https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki
uint32_t nTime=DELAY; //неделя в секундах
uint32_t nSequence=(1<<22) | (nTime>>9);
RSMCScript.push_back(operation(uint32_to_data_chunk(nSequence)));

RSMCScript.push_back(operation(opcode::checksequenceverify));
RSMCScript.push_back(operation(opcode::drop));
RSMCScript.push_back(operation(to_chunk(ServerPublicKey.point())));
RSMCScript.push_back(operation(opcode::endif));
RSMCScript.push_back(operation(opcode::checksig));

output_point UTXO(tx.hash(), 0u);


if( !validate_tx(remote_commit_tx) || //проверяем на то, что подписанная транзакция валидна
    remote_commit_tx.inputs()[0].previous_output() != UTXO || //вход полученной транзакции равен вход сгенерированной самим сервером транзакции(выход открывающей транзакции)
    remote_commit_tx.outputs()[0].value() != ClientBalance - (ClientBalance * CLOSING_CHANNEL_FEES/(MyBalance+ClientBalance)) ||
    remote_commit_tx.outputs()[1].value() != MyBalance - (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+ClientBalance))    ||
    remote_commit_tx.outputs()[0].script()!=script::to_pay_key_hash_pattern(ClientPublicKey.to_payment_address().hash()) || //скрипт первого выхода
    remote_commit_tx.outputs()[1].script()!=RSMCScript //скрипт RSMC выхода
   )
{
    //если хоть одна проверка не пройдена
    //отправляем в сеть последнюю полученную от пользователя транзакцию
    std::cout<<"ERROR: First commitment transaction invalid\n";
    return;
}



//ждем сигнала от клиента либо о закрытие канала(1), либо о том, что он желает совершить платеж по каналу
int8_t message;
uint64_t summ; //в переменную пишем суммы платежей
std::vector<ec_secret> remote_per_commitment_secret; //массив предыдущих секретов клиента
std::vector<transaction> prev_commit_transactions; //для проверки, если пользователь отправил предыдущие транзакции в сеть, то их можно найти по хэшу
std::vector<hash_digest> prev_commit_transactions_hash; //для проверки, если пользователь отправил предыдущие транзакции в сеть, то их можно найти по хэшу

std::cout<<"waiting command from client\n";

uint index;
while(true)
{

    //ждем сообщения от пользователя
    while(socket->receive(boost::asio::buffer(&message, sizeof(int8_t))) != sizeof(int8_t)){}

    //так как операция проверки, на то входят ли все предыдущие транзакции довольно долгая, то чтобы не замедлять отклик на команды пользователя, ее бы лучше вынести в отдльный поток, но я чтоб пока не заморачиваться с межпотоковой передачей данных буду делать все в одном потоке, просто проверять все предыдущие транзакции после каждого сообщения от клиента
    //такая система совсем не подходит, например для посекундной оплаты видео в потоке, так как проверка предыдущих транзакции занимает более нескольких секунд времени(в зависимости от величины массива предыдущих транзакции), то видео для пользователя в потоке зависало бы, периодически на несколько секунд, что неприемлемо для такого сервиса, к примеру. В реальном же сервисе тогда нужно будет вывести такую проверку в отдельный поток
    std::cout<<"checking...\n\n";
    int i=check_all_prev_tx(prev_commit_transactions_hash);
    if(i>=0) //найден индекс транзакции которая попала в блокчейн
    {
        std::cout<<"client broadcasted old transaction with index: "<<i<<std::endl;
        index=i;
        message=-1;
    }

    if(message==0) //клиента хочет совершить платеж
    //ожидаем от клиента суммы платежа, меняем балансы и создаем новую транзакцию обязательство
    {
        while(socket->receive(boost::asio::buffer(&summ, sizeof(uint64_t))) != sizeof(uint64_t)){}

        //получаем секретное значение для предыдущей транзакции обязательства от клиента
        ec_secret current_remote_per_commitment_secret;
        while(socket->receive(boost::asio::buffer(&current_remote_per_commitment_secret, sizeof(ec_secret))) != sizeof(ec_secret)){}
        std::cout<<"client prev per_commitment_secret:\n\n"<< encode_base16(current_remote_per_commitment_secret)<<std::endl;

        //проверяем, чтоб отправленный секретный ключ соответсвовал полученной ранее точке
        ec_compressed current_remote_per_commitment_basepoint(gen_point);
        ec_multiply(current_remote_per_commitment_basepoint, current_remote_per_commitment_secret);
        if(remote_per_commitment_basepoint!=current_remote_per_commitment_basepoint || //если отправленный ключ не соответствует отправленному ранее значению
           summ>ClientBalance          //если суммма платежа больше чем баланс клиента внутри канала
          )
        {
            //транслируем транзакцию-обязательство в сеть (она уже была подписана нами ранее)
            broadcast_tx(remote_commit_tx);

            //послн транслирования транзакции клиент сразу получает средства, а для сервера создадим транзакцию транслирую которую через время он сможет забрать средства
            transaction timeunlock_tx= create_after_timelock_transaction(remote_commit_tx, ServerPublicKey, ServerSecret);
            //такая транзакция может быть отправлена только спустя значение nSequence определенное в секундах , в алгоритме лайтинга довольно большон число неделя ии две,
            //в реальном сервисе стоило бы сохранять код такой транзакции в какой-либо лог-файл вместе с метками времени, в которые можно будет отправлять такую транзакцию в сеть
            //но пока просто выведем в консоль код транзакции и предупреждение о том, что таку транзакцию необходимо отправить в сеть через неделю
            std::cout<<"channel was closed, please send this tx in blockchain after one week:\n"<<encode_base16( timeunlock_tx.to_data() )<<std::endl;
            return;
        }


        //иначе (сумма платежа меньше баланса и точка валидна) идем далее
        remote_per_commitment_secret.push_back(current_remote_per_commitment_secret); //добавляем в новый секрет
        ClientBalance-=summ;
        MyBalance+=summ;
        std::cout<<"client doing payment on value "<<summ<<std::endl;

        //получаем новую точку для новрй транзакции-обязательства per_commitment_basepoint
        while(socket->receive(boost::asio::buffer(&remote_per_commitment_basepoint, ec_compressed_size)) != ec_compressed_size){}
        std::cout<<"received new per_commitment_basepoint\n\n";
        //отправляем свой ключ, для своей предыдущей транзакции обязательства(необязательно для однонаправленного канала, так как серверу в однонаправленном канале, все равно не выгодно транслировать в сеть старые транзакции)

        //получим от клиента хэш предыдущей транзакции обязательства - необходимо для проверки того входит ли такая транзакция в блокченн или нет, если входит, то клиент пытается обманть сервер, создадим транзакцию penalty и отправим в сеть
        //такой способ проверки должен быть в дальнейшем доработан, а иименно дело в том, что клиент может отправить хэш не от транзакции, а у сервера нету возможность проверить его
        //если сервер является полной биткоин нодой, то для такой проверки будет неободимо пробегать по всему блокчейну(или лучше по поседним блокам) в поисках транзакции со входом, выходом для которого является выход funding transaction
        //пока же сервер не является полно нодой, а сам отправляет запросы серверу libbitcoin будем делать так(получать хэш)
        hash_digest cur_hash;
        while(socket->receive(boost::asio::buffer(&cur_hash, hash_size)) != hash_size){}

        //сохраним хэш от своей предыдщей транзакции обязательства
        prev_commit_transactions_hash.push_back(cur_hash);
        prev_commit_transactions.push_back(commitTX);

        //создаем новую транзакцию обязательство
        commitTX=create_commitment_tx(tx,ClientPublicKey,ServerSecret,revocation_basepoint,remote_per_commitment_basepoint,MyBalance,ClientBalance);
        //отправляем клиенту свою транзакцию обязательство (в однонаправленном канал сервер может использовать один и тот же секретный ключ, при условие, что не передает его клиенту)
        send_transaction(commitTX,socket);

        //получаем от клиента его транзакцию обазательство
        transaction current_remote_commit_transaction = receive_transaction(socket);
        std::cout<<"was received new commit-tx form client\n\n";

        //проверяем полученную транзакцию обязательство(соответсвие входа выходу образующей транзакции канала, и выходных балансов. а также то, что для выходов клиент действительно использовал переданную ему от сервера отдоразовую точку)
       //сперва подпишем полученную транзакцию
        endorsement Sig0;
        operation::list sig_script0;
        script::create_endorsement(Sig0,ServerSecret, tx.outputs()[0].script(),  current_remote_commit_transaction,0,0x01);

        sig_script0.push_back(operation(opcode::push_size_0));
        sig_script0.push_back(operation(Sig0));
        sig_script0.push_back(current_remote_commit_transaction.inputs()[0].script().operations()[0]);
        script InputScript0(sig_script0);
        current_remote_commit_transaction.inputs()[0].set_script(InputScript0);

        //создадим скрипт, такой же, какой должен был создать клиент для RSMC выхода, создадим его для сравнения его со скриптом полученным в подписанной пользователем транзакции.
        ec_compressed remote_revocation_pub_key;
        point_list commitment_point_list = {per_commitment_basepoint,remote_revocation_basepoint};
        //revocation_pub_key= remote_per_commitment_basepoint + revocation_basepoint. в поле эллиптических точек.
        ec_sum(remote_revocation_pub_key, commitment_point_list);
        ec_multiply( remote_revocation_pub_key, to_array<ec_secret_size>( sha256_hash_chunk( XOR(remote_revocation_basepoint, per_commitment_basepoint) )) );

        operation::list RSMCScript;
        RSMCScript.push_back(operation(opcode::if_));
        RSMCScript.push_back(operation(to_chunk(remote_revocation_pub_key)));
        RSMCScript.push_back(operation(opcode::else_));
        //добавить to_self_delay

        RSMCScript.push_back(operation(uint32_to_data_chunk(nSequence)));

        RSMCScript.push_back(operation(opcode::checksequenceverify));
        RSMCScript.push_back(operation(opcode::drop));
        RSMCScript.push_back(operation(to_chunk(ServerPublicKey.point())));
        RSMCScript.push_back(operation(opcode::endif));
        RSMCScript.push_back(operation(opcode::checksig));


        if( !validate_tx(current_remote_commit_transaction) || //проверяем на то, что подписанная транзакция валидна
            current_remote_commit_transaction.inputs()[0].previous_output() != remote_commit_tx.inputs()[0].previous_output() || //вход полученной транзакции равен вход сгенерированной самим сервером транзакции(выход открывающей транзакции)
            current_remote_commit_transaction.outputs()[0].value() != ClientBalance - (ClientBalance * CLOSING_CHANNEL_FEES/(MyBalance+ClientBalance)) ||
            current_remote_commit_transaction.outputs()[1].value() != MyBalance - (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+ClientBalance))    ||
            current_remote_commit_transaction.outputs()[0].script()!=script::to_pay_key_hash_pattern(ClientPublicKey.to_payment_address().hash()) || //скрипт первого выхода
            current_remote_commit_transaction.outputs()[1].script()!=RSMCScript //скрипт RSMC выхода
           )
        {
            //если хоть одна проверка не пройдена
            //отправляем в сеть последнюю полученную от пользователя транзакцию
            std::cout<<"ERROR: commitment transaction invalid\n";

            if( !broadcast_tx(remote_commit_tx))
            {
                //если не удается передать последнюю транзакцию- обязательство, нужно просмотреть ошибку из-за которой не удается передать, и действовать по обстоятельствам
                //например, если получена ошибка о том, что выход уже потрачен, значит клиент отправил транзакцию-обязательство в сеть
                //тогда нужно узнать отправил ли он последнюю транзакцию или нет, и по обстоятельствам или не делать ничего или или забрать свои барыши используя секретное значение полученное пользователем от предыдущей транзакции
                //пока оставим обработку таких случаев на потом, хотя они являются неотъемленной частью Lighting network
            }

            //послн транслирования транзакции клиент сразу получает средства, а для сервера создадим транзакцию транслирую которую через время он сможет забрать средства
            transaction timeunlock_tx= create_after_timelock_transaction(remote_commit_tx, ServerPublicKey, ServerSecret);
            //такая транзакция может быть отправлена только спустя значение nSequence определенное в секундах , в алгоритме лайтинга довольно большон число неделя ии две,
            //в реальном сервисе стоило бы сохранять код такой транзакции в какой-либо лог-файл вместе с метками времени, в которые можно будет отправлять такую транзакцию в сеть
            //но пока просто выведем в консоль код транзакции и предупреждение о том, что таку транзакцию необходимо отправить в сеть через неделю
            std::cout<<"channel was closed, please send this tx in blockchain after one week:\n"<<encode_base16( timeunlock_tx.to_data() )<<std::endl;
            return;
        }

        //после удачных провероу заменяем, если проверки неудачны отправляем последнюю полученную транзакцию обязательство в сеть и считаем платеж неудавжимся, а канал закрытым
        remote_commit_tx=current_remote_commit_transaction; //старая транзакция обязательство становится ненужной

        std::cout<<"waiting command from client\n";
    }

    if(message==1)//клиент желает закрыть канал
    {
        //получаем от клиента подписанную транзакцию, созданную на вывод текущих балансов без временных задержек, на адрес сервера и свой
        transaction lastTX=receive_transaction(socket);
        //проверяем эту транзакцию на соответсвие балансов, адресов и тд
        if(
            lastTX.outputs()[0].value() != ClientBalance - (ClientBalance * CLOSING_CHANNEL_FEES/(MyBalance+ClientBalance)) ||
            lastTX.outputs()[1].value() != MyBalance - (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+ClientBalance))    ||
            lastTX.outputs()[1].script()!= script::to_pay_key_hash_pattern(ServerPublicKey.to_payment_address().hash())
           )
        {
            //закроем канал другим способом, тем что подпишем последнюю полученную от клиента транзакцию обязательство и отправим в сеть
            if( !broadcast_tx(remote_commit_tx))
            {
                //если не удается передать последнюю транзакцию- обязательство, нужно просмотреть ошибку из-за которой не удается передать, и действовать по обстоятельствам
                //например, если получена ошибка о том, что выход уже потрачен, значит клиент отправил транзакцию-обязательство в сеть
                //тогда нужно узнать отправил ли он последнюю транзакцию или нет, и по обстоятельствам или не делать ничего или или забрать свои барыши используя секретное значение полученное пользователем от предыдущей транзакции
                //пока оставим обработку таких случаев на потом, хотя они являются неотъемленной частью Lighting network
            }

            //после отправки последней клиентской транзакции-обязательства составим транзакция для вывода монет на свой адрес
            transaction timeunlock_tx=create_after_timelock_transaction(remote_commit_tx, ServerPublicKey,ServerSecret);

            //после создания такой транзакции выведем в консоль сервера ее код в base16, и предупредил пользователя о том, что ему нужно отправить данную транзакцию в сеть через семь дней
            std::cout<<"Channel closed! please broadcast it tx in blockchain after 7 days:\n"<<encode_base16( timeunlock_tx.to_data() )<<std::endl;

            //на реальном же сервере наверное лучше было бы записать такой код в лог файл вместе со таймстемпами, и реализовать автоматическую отправку этих транзакции в сеть по истечению времени. но я остановлюсь для начала на варианте с консолью.
            return;
        }
        //если транзакция прошла все проверки
        //подписываем транзакцию свойм приватным ключом и транслируем в сеть
        std::cout<<"tx code:\n"<< encode_base16( lastTX.hash() )<<std::endl;

        endorsement Sig0;
        operation::list sig_script0;
        script::create_endorsement(Sig0,ServerSecret, tx.outputs()[0].script(),  lastTX,0,0x01);

        sig_script0.push_back(operation(opcode::push_size_0));
        sig_script0.push_back(operation(Sig0));
        sig_script0.push_back(lastTX.inputs()[0].script().operations()[0]);
        script InputScript0(sig_script0);
        lastTX.inputs()[0].set_script(InputScript0);

        std::cout<<"tx code:\n"<<encode_base16( lastTX.hash())<<std::endl;

        std::cout<<"tx code:\n"<<encode_base16( lastTX.to_data())<<std::endl;
        //транслируем транзакцию в сеть
        if( !broadcast_tx(lastTX))
        {
            //если не удается передать последнюю транзакцию- обязательство, нужно просмотреть ошибку из-за которой не удается передать, и действовать по обстоятельствам
            //например, если получена ошибка о том, что выход уже потрачен, значит клиент отправил транзакцию-обязательство в сеть
            //тогда нужно узнать отправил ли он последнюю транзакцию или нет, и по обстоятельствам или не делать ничего или или забрать свои барыши используя секретное значение полученное пользователем от предыдущей транзакции
            //пока оставим обработку таких случаев на потом, хотя они являются неотъемленной частью Lighting network
        }
        return;//выходим из цикла, программа закрывается
    }

    if(message==-1) //была найдена транзакции отправленная клиентом в блокчейн, из предыдущихтранзакции-обязательство, то есть со старым балансом
    {
        //создадим транзакции которая отправит себе все средства с транзакции
        //и протранслируем ее в блокчейн
        transaction penalty_tx;
        penalty_tx.set_version(2u);

        //вход
        output_point RSMC_Out(prev_commit_transactions_hash[index], 1u); //вход транзакции обязательства, это первый(с нулевым индексом) выход созданной ранее транзакции

        input input0;
        input0.set_previous_output(RSMC_Out);
        input0.set_sequence(0xffffffff);
        operation::list inputScript;
        inputScript.push_back(operation(opcode::push_positive_1));
        input0.set_script(inputScript);
        penalty_tx.inputs().push_back(input0); //добавим вход без подписи

        //выходы
        output output0;  //выход на свой адрес с p2pkh выходом
        output0.set_value(prev_commit_transactions[index].outputs()[1u].value() - CLOSING_CHANNEL_FEES);
        operation::list p2pkhScript=script::to_pay_key_hash_pattern(ServerPublicKey.to_payment_address().hash());
        output0.set_script(p2pkhScript);
        penalty_tx.outputs().push_back(output0);

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
    revocationprivatekey = revocation_secret * SHA256(revocation_basepoint || per_commitment_point) + per_commitment_secret * SHA256(per_commitment_point || revocation_basepoint)
     , где || операция XOR(исключающее или)
     */
        ec_secret revocation_private_key(revocation_basepoint_secret);
       // ec_secret remote_per_commitment_secret[index];
        ec_compressed penalty_remote_per_commitment_base_point(gen_point);
        ec_multiply( penalty_remote_per_commitment_base_point, remote_per_commitment_secret[index] );

        ec_add(revocation_private_key, remote_per_commitment_secret[index]);

        ec_multiply( revocation_private_key, to_array<ec_secret_size>( sha256_hash_chunk( XOR(revocation_basepoint, penalty_remote_per_commitment_base_point) )) );

        //подпишем транзакцию
        endorsement Sig0;
       // считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
        script::create_endorsement(Sig0, revocation_private_key, prev_commit_transactions[index].outputs()[1u].script(),  penalty_tx,0,0x01);

        operation::list  sig_script0;
        sig_script0.push_back(operation(Sig0));
        sig_script0.push_back(operation(opcode::push_positive_1));
        script InputScript0(sig_script0);
        penalty_tx.inputs()[0].set_script(InputScript0);

        //отправим в сеть транзакцию
        if(!broadcast_tx(penalty_tx))
        {
            //если транзакция не проходит, то нужно смотреть на ошибки и реагировать по ситуации
            std::cout<<"tx code:"<<encode_base16( penalty_tx.to_data() )<<std::endl;
            std::cout<<"EROR PENALTY\n\n";
            return;
        }
        std::cout<<"PENALTY TRANSACTION BROADCASTED\n\n";
        break;
    }
}


}

int main() {

  boost::asio::io_service IOservice;
  boost::asio::ip::tcp::endpoint Endpoint(boost::asio::ip::tcp::v4(),17666); //порт 17666, ждем соединения от любого ipv4
  boost::asio::ip::tcp::acceptor Acceptor(IOservice, Endpoint);
  boost::asio::ip::tcp::socket* Socket=new boost::asio::ip::tcp::socket(IOservice);
  Acceptor.async_accept(*Socket,std::bind( server_handler, Socket, std::placeholders::_1));
  IOservice.run();

  return 0;

}
