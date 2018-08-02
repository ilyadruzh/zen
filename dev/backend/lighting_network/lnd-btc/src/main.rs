// Additional information about types -
//      http://libbitcoin.github.io/doc/overview.html
//      https://github.com/libbitcoin/libbitcoin/blob/3584257aed08f64afbf4218ecd711315617fb3b5/include/bitcoin/bitcoin/math/elliptic_curve.hpp

extern crate bitcoin;
extern crate bitcoin_bech32;
extern crate bitcoin_rpc_json;
extern crate serde;
extern crate tokio;

use serde::ser::{Serialize, Serializer};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, AddrParseError};
use std::io::prelude::*;
use std::io::{self, Read, Write, ErrorKind};
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

// https://github.com/libbitcoin/libbitcoin/blob/fdb5558e5fa03c0afbb0e447c69757869c7f148a/include/bitcoin/bitcoin/chain/transaction.hpp
use bitcoin::blockdata::transaction;

use bitcoin::network::serialize::*;

// Use to pass an ec point as either ec_compressed or ec_uncompressed.
// ec_public doesn't carry a version for address creation or base58 encoding.
// https://github.com/libbitcoin/libbitcoin/blob/fdb5558e5fa03c0afbb0e447c69757869c7f148a/include/bitcoin/bitcoin/wallet/ec_public.hpp
pub type ec_public = u64;

// type ec_compressed: byte_array<ec_compressed_size>
// static BC_CONSTEXPR size_t ec_compressed_size = 33;
// https://github.com/libbitcoin/libbitcoin/blob/cf4f3889fe26372982d6786bbb4f9e7953f66484/include/bitcoin/bitcoin/compat.hpp
pub type ec_compressed = Vec<usize>;

// Raw bytes. A chunk of data. This type is equivalent to std::vector<uint8_t>.
pub type data_chunk = Vec<u8>;

// static BC_CONSTEXPR size_t ec_secret_size = 32;
// typedef byte_array<ec_secret_size> ec_secret;
pub type ec_secret = Vec<usize>;

const CLOSING_CHANNEL_FEES: usize = 10000;
const DELAY: usize = 600;
const EC_COMPRESSED_SIZE: usize = 33;
const EC_SECRET_SIZE: usize = 32;

// переводит u32 в вектор u8
// не работает - изучить побитовый сдвиг вправо
fn u32_to_data_chunk(value: u32) -> data_chunk {
    let mut return_value: data_chunk = Vec::new(); // 32 бита
    for x in (0..3).rev() {
        return_value.push((value >> (8 * x)) as u8);
    }
    println!("return_value in u32_to_data_chunk: {:?}\n", &return_value);

    return return_value;
}

// побитовое
fn XOR(point1: ec_compressed, point2: ec_compressed) -> ec_compressed {
    let mut return_value: ec_compressed = Vec::new();

    for x in (0..EC_COMPRESSED_SIZE) {
        return_value[x] = point1[x] ^ point2[x];
    }

    println!("return_value in XOR: {:?}\n", &return_value);

    return return_value;
}

// отправка транзакции
fn send_transaction(tx: transaction::Transaction, socket: TcpStream) {
    let sender_data: data_chunk = bitcoin::network::serialize::serialize(&tx).unwrap(); // обработать ошибку
    println!("sender_data: {:?}", &sender_data);
    let size_data = sender_data.len();

    // socket->send(boost::asio::buffer(&SizeData,sizeof(uint64_t)));
    // for(int i=0; i < SizeData; i++) {
    //    socket->send( boost::asio::buffer(&SenderData[i],sizeof(char)) );
    // }

    // Отправка на сокет буфера, который содержит размер данных(?)
    // socket->send(boost::asio::buffer(&SizeData,sizeof(uint64_t)));
    // socket.write(&size_data);

    // for i in (0..size_data) {
    //     // отправка на соект буфера с перебором элемента массива sender_data размера char(?)
    //     // socket->send( boost::asio::buffer(&SenderData[i],sizeof(char)) );
    //     socket.write(sender_data[i] as u8);
    // }
}

// Получение транзакции
// socket: boost::asio::ip::tcp::socket
fn receive_transaction(socket: TcpStream, tx1: transaction::Transaction) -> transaction::Transaction {
    let size_of_str: u64;
  
    //ждем пока клиент отправит первые данные, размер транзакции в символах в кодировке base16
    // while (
    //     socket->receive(
    //         boost::asio::buffer(size_of_str, sizeof(uint64_t)) 
    //     ) != sizeof(uint64_t)){};
    
    //получаем транзакцию (base16 код в строку)
    let tx_data: data_chunk;
    let cur_byte: u8;

    // for i in (0..size_of_str) {
    //             socket->receive(boost::asio::buffer(cur_byte, sizeof(uint8_t)));
    //     tx_data.push_back(cur_byte);
    // }

    // комментарий - std::cout<<Tx_str<<std::endl;
    // let tx: transaction::Transaction = bitcoin::network::serialize::deserialize(tx_data);
    // tx.from_data(tx_data,true,true);

    return tx1;

}

// Создание открывающей канал транзакции
fn create_opening_tx(server_public_key: ec_public, ClientPubKey: ec_public, user_secret: ec_secret) -> transaction::Transaction {
    let client_address: payment_address = ClientPubKey.to_payment_address();

    //создаваемая транзакция
    let tx: transaction::Transaction;
    // old - tx.set_version(1u);
    tx.set_version(1);

    //входы транзакции(в идеале лучше было бы, еслиб программа сама находила и строила входы, как например с помощью функции getUTXO, суть ее в том, чтобы обраться к серверу(в данном случае libbitcoin серверу) и отправить запрос на получение данных из блокчейна, из которые уже можно бы было получить UTXO для конкретного адреса,но сервер возвращает пустое множество UTXO, для любого адреса. долгое время не смогя разобраться с этой проблемой, было решено пока возложить на пользователя задачу найти и вписать свой UTXO)
    //выходы можно самостоятельно посмотреть на сайте
    //https://live.blockcypher.com/btc-testnet/tx/62408b1b14ce9eea82b73b543cfb0bdfc4ec118b9d50c07e6c6d75ba3c6a7b59/
    //тут необходимо ввести хэш транзакции в которой есть непотраченный выход

    let prev_tx_hash_str: String = String::new();

    println!("\n write your prev UTXO hash in base16, once output from this will input for channel's creation's transaction:\n");
    // Ввод данных в консоль
    // std::cin>>prev_tx_hash_str;
    io::stdin().read_to_string(&mut prev_tx_hash_str);

    // Неизвестный тип hash_digest
    let prev_tx_hash: hash_digest;

    decode_hash(prev_tx_hash, prev_tx_hash_str);

    //тут нужно ввести индекс непотраченного выхода, индексация начинается с нуля, выходы например на сайте
    //https://live.blockcypher.com/btc-testnet/tx/62408b1b14ce9eea82b73b543cfb0bdfc4ec118b9d50c07e6c6d75ba3c6a7b59/
    //расположены сверху внизу в порядке увеличения их индекса
    let mut prev_tx_index: u32;
    println!("\n write index unspended output of your UTXO, its output will be input:\n");
    // Ввод данных в консоль
    // std::cin>>prev_tx_index;
    io::stdin().read_to_string(&mut prev_tx_index);

    //желательно реализовать поиска непотраченных выходов на определенную сумму автоматически,
    //то есть просить пользователя ввести только адрес и уже пробегая по блокчейну искать выходы
    //в предположение, что машина пользователя не является полной биткоин нодой,
    //нужно отправлять запрос серверу для нахождения непотраченных выходов.
    //но так как программа писалась для демонстрации работы Lighing канала, было решено поиск непотраченных выходов временно оставить на пользователе

    let mut weight_of_channel_btc: String = String::new();;
    let weight_of_channel_satoshi: u64 = 0;
    
    println!("\n write weight of channel in BTC:\n");
    // Ввод данных в консоль
    // std::cin>>weight_of_channel_btc;
    io::stdin().read_to_string(&mut weight_of_channel_btc);
    decode_base10(weight_of_channel_satoshi, weight_of_channel_btc, btc_decimal_places);

    //сдача пользователю, то есть открывающая транзакция (opening_tx) имеет 2 выхода - первый, на счет с мультподписью размер отправляеных на него биткоинов равен ширине канала, второй - сдача пользователю, остаток средств которые он хочет вернуть на свой адрес, он наберет их сам, учитывая какую сумму он хочет потратить на fees
    let mut odd_money_btc: String = String::new();;
    let odd_money_satoshi: u64 = 0;
    println!("\n write odd money (in BTC), for creating p2pkh output on your address. Dont forgot about transaction's fees:\n");
    // Ввод данных в консоль
    // std::cin>>odd_money_btc;
    io::stdin().read_to_string(&mut odd_money_btc);

    decode_base10(odd_money_satoshi, odd_money_btc, btc_decimal_places);

    output_point UTXO(prev_tx_hash, prev_tx_index);

    input input0;
    input0.set_previous_output(UTXO);
    input0.set_sequence(0xffffffff);

    //добавим вход без подписи
    tx.inputs().push_back(input0);

    //создадим выходы транзакции (2 выхода:
    //первый - на адрес с мультиподписью 2из2)
    //второй - сдача самому себе, выход со скриптом вида p2pkh

    point_list PubKeys({server_public_key.point(), ClientPubKey.point()});
    //выход с мультиподписью
    operation::list MultisigLockingScript=script::to_pay_multisig_pattern(2u,PubKeys); 
    /*
    MultisigLockingScript.push_back(operation(opcode::push_positive_1));
    MultisigLockingScript.push_back(operation(to_chunk(server_public_key.point() )));
    MultisigLockingScript.push_back(operation(to_chunk(ClientPubKey.point() )));
    MultisigLockingScript.push_back(operation(opcode::push_positive_2));
    MultisigLockingScript.push_back(operation(opcode::checkmultisig));
    */
    //вызываем конструктор класса output, в который передаем 1 - количество коинов, 2 - скрипт
    output Output0(weight_of_channel_satoshi, MultisigLockingScript); 

    //скрипт для возвращает сдачу себе
    let p2pkhScript: operation::list = script::to_pay_key_hash_pattern(client_address.hash()); 

    //создаем второй выход
    output Output1(odd_money_satoshi, p2pkhScript);

    tx.outputs().push_back(Output0);
    tx.outputs().push_back(Output1);

    //после добавления всех входов и выходов подпишем транзакцию

    let sig_0: endorsement;
    
    // считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
    script::create_endorsement(sig_0, user_secret, p2pkhScript, tx, 0, 0x01);

    let sig_script0: operation::list;
    sig_script0.push_back(operation(sig_0));
    sig_script0.push_back(operation(to_chunk(ClientPubKey.point() )));
    script InputScript0(sig_script0);
    tx.inputs()[0].set_script(InputScript0);

    return tx;
}

fn create_commitment_tx(
    funding_tx: transaction,
    server_public_key: ec_public, 
    user_secret: ec_secret,
    revocation_basepoint: ec_compressed,
    remote_per_commitment_basepoint: ec_compressed,
    my_balance: u64, 
    server_balance: u64) {

    ec_private UserPrivate(user_secret, ec_private::testnet);
    let user_pub_key: ec_public = UserPrivate.to_public();

    let commit_tx: transaction::Transaction;
    commit_tx.set_version(2u);

    //вычислим n_sequence(необходим для задержки валюты), как описано на:
    //https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki

    let n_time: u32 = DELAY; //неделя в секундах
    let n_sequence: u32 =(1<<22) | (n_time>>9);

    //вход
    output_point UTXO(funding_tx.hash(), 0u); //вход транзакции обязательства, это первый(с нулевым индексом) выход созданной ранее funding транзакции

    let input_0: input;
    input_0.set_previous_output(UTXO);
    input_0.set_sequence(0xffffffff);

    commit_tx.inputs().push_back(input_0); //добавим вход без подписи

    //выходы
    let output_0: output;  //выход на свой адрес с p2pkh выходом
    output_0.set_value(my_balance - (my_balance * CLOSING_CHANNEL_FEES / (my_balance + server_balance)));
    let p2pkhScript: operation::list = script::to_pay_key_hash_pattern(user_pub_key.to_payment_address().hash());
    output_0.set_script(p2pkhScript);

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

    let revocation_pub_key: ec_compressed;
    let commitment_point_list: point_list = {remote_per_commitment_basepoint,revocation_basepoint};

    //revocation_pub_key= remote_per_commitment_basepoint + revocation_basepoint. в поле эллиптических точек.
    ec_sum(revocation_pub_key, commitment_point_list);
    ec_multiply( revocation_pub_key, to_array<ec_secret_size>( sha256_hash_chunk( XOR(revocation_basepoint, remote_per_commitment_basepoint) )) );

    let output1: output;
    output1.set_value(server_balance - (server_balance * CLOSING_CHANNEL_FEES / (my_balance + server_balance)));

    let output_script_1: operation::list;
    output_script_1.push_back(operation(opcode::if_));
    output_script_1.push_back(operation(to_chunk(revocation_pub_key)));
    output_script_1.push_back(operation(opcode::else_));

    //добавить to_self_delay
    output_script_1.push_back(operation(uint32_to_data_chunk(n_sequence)));

    output_script_1.push_back(operation(opcode::checksequenceverify));
    output_script_1.push_back(operation(opcode::drop));
    output_script_1.push_back(operation(to_chunk(server_public_key.point())));
    output_script_1.push_back(operation(opcode::endif));
    output_script_1.push_back(operation(opcode::checksig));

    output1.set_script(output_script_1);

    commit_tx.outputs().push_back(output_0);
    commit_tx.outputs().push_back(output1);

    //подпишем транзакцию своим приватным ключом
    let sig_0: endorsement;
    
    //считаем скрипт выхода соответствующнго входу0 равен p2pkhScript
    script::create_endorsement(sig_0, user_secret, funding_tx.outputs()[0].script(), commit_tx, 0, 0x01);

    let sig_script0: operation::list;
    sig_script0.push_back(operation(sig_0));
    script InputScript0(sig_script0);
    commit_tx.inputs()[0].set_script(InputScript0);

    return commit_tx;
}

// TcpStream - boost::asio::ip::tcp::socket
// AddrParseError - boost::system::error_code
fn client_handler(socket: TcpStream, ec: AddrParseError) {
    if (ec) {
        // вывод информации по ошибке
        std::cout<<ec.message()<<std::endl;
        return;
    }

    ec_public ServerPublicKey("033ddf60c1191d4e16e4f59a6fa3b5e899c47915a634460d2add2b6bdec0b4c3c6"); //адрес сервера нужно по идее передавать по сети

    let mut client_pub_key_str: String = String::new();
    println!("enter your public key in base16:\n");
    // Ввод данных в консоль
    // std::cin>>client_pub_key_str;
    io::stdin().read_to_string(&mut client_pub_key_str);

    ec_public ClientPubKey(client_pub_key_str);

    //для подписи требуется приватный ключ пользователя
    let user_secret_str: String = String::new();
    println!("write your private key in base16 (it need for sifnaturing of tx, and doesnt be send on server):\n");
    // Ввод данных в консоль
    // std::cin>>user_secret_str;
    io::stdin().read_to_string(&mut user_secret_str);

    let user_secret: ec_secret;
    decode_base16(user_secret, user_secret_str);
    ec_private UserPrivate(user_secret, ec_private::testnet);

    if (UserPrivate.to_public() != ClientPubKey) {
        println!("error: Invalid private key for inputed before public key\n");
    }

    transaction funding_tx=create_opening_tx(ServerPublicKey, ClientPubKey, user_secret);

    //cout<<"\n\n tx: "<<encode_base16(tx.to_data(true,true)) << std::endl;

    //отправляем транзакцию серверу для проверки на наличие выхода с мультиподписью и для трансляции в блокчейн
    send_transaction(funding_tx,socket);
    // вывод данных в консоль
    std::cout<<"fundibg tx hash: \n"<< encode_base16( funding_tx.hash() )<<std::endl;

    //ждем ответ от сервера
    
    let returnerFlag: bool;
    while (socket->receive( boost::asio::buffer(&returnerFlag, sizeof(bool)) ) != sizeof(bool)){
        //ждем пока клиент отправит первые данные, размер транзакции в символах в кодировке base16
    }; 

    if (!returnerFlag) {
        println!("Error: Transaction is not valid.\n");
        return;
    }
    println!("Transaction broadcasted in Blockchain, now creating commit-transaction\n\n");

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
    let revocation_basepoint_secret: ec_secret = to_array<ec_secret_size>(my_entropy);
    ec_compressed revocation_basepoint(gen_point);

    ec_multiply(revocation_basepoint, revocation_basepoint_secret);

    //генерируем per_commitment_basepoint, и соответсвенно per_commitment_secret
    pseudo_random_fill(my_entropy);
    let per_commitment_secret: ec_secret = to_array<ec_secret_size>(my_entropy);
    ec_compressed per_commitment_basepoint(gen_point);
    ec_multiply(per_commitment_basepoint, per_commitment_secret);

    //отправляем revocation_basepoint серверу
    socket->send(boost::asio::buffer(&revocation_basepoint, ec_compressed_size));

    let remote_revocation_basepoint: ec_compressed;
    //получаем от сервера remote_revocation_basepoint
    while (socket->receive(boost::asio::buffer(&remote_revocation_basepoint, ec_compressed_size)) != ec_compressed_size){}

    //отправляем per_commitment_basepoint серверу
    socket->send(boost::asio::buffer(&per_commitment_basepoint, ec_compressed_size));
    let remote_per_commitment_basepoint: ec_compressed;

    //получаем от сервера remote_per_commitment_basepoint
    while (socket->receive(boost::asio::buffer(&remote_per_commitment_basepoint, ec_compressed_size)) != ec_compressed_size){}

    //внутри-канальные балансы
    let MyBalance: u64 = funding_tx.outputs()[0].value()-1000u;
    let ServerBalance: u64 = 1000u;

    //создаем транзакцию-обязательство

    //создаем первую транзакцию обязательство
    let commit_tx: transaction = create_commitment_tx(funding_tx,ServerPublicKey, user_secret, revocation_basepoint, remote_per_commitment_basepoint,MyBalance,ServerBalance);

    //отправляем серверу первую транзакцию-обязательство
    send_transaction(commit_tx,socket);

    //получаем от сервера его транзакцию
    let remote_commit_tx: transaction = receive_transaction(socket);

    //подпишем полченную транзакцию обязательство
    let Sig0: endorsement;
    let sig_script0: operation::list;
    script::create_endorsement(Sig0, user_secret, funding_tx.outputs()[0].script(),  remote_commit_tx,0,0x01);

    sig_script0.push_back(operation(opcode::push_size_0));
    sig_script0.push_back(remote_commit_tx.inputs()[0].script().operations()[0]);
    sig_script0.push_back(operation(Sig0));
    script InputScript0(sig_script0);
    remote_commit_tx.inputs()[0].set_script(InputScript0);

    // вывод данных в консоль
    std::cout<<"my coomit tx:\n"<<encode_base16(commit_tx.to_data(true,true))<<std::endl;
    std::cout<<"server coomit tx:\n"<<encode_base16(remote_commit_tx.to_data(true,true))<<std::endl;

    println!("channel was created,");

    //обмениваемся с сервером сообщениями

    let mut command: String = String::new();
    let message: i8 = 0;
    let sum: u64;

    while (message!=1) {
        println!(" write \"-pay\" for payment by channel, or \"-close\" for closing of channel\n");
        // Вывод данных в консоль
        std::cout<<"your in-channel Balance: "<< encode_base10(MyBalance, btc_decimal_places)<<std::endl;
        std::cout<<"servet in-channel Balance: "<<encode_base10(ServerBalance, btc_decimal_places)<<std::endl;
        // Ввод данных в консоль
        // std::cin>>Command;
        io::stdin().read_to_string(&mut command);

        //хотим отправить серверу плату
        if (command == "-pay") {

            message = 0;
            println!("write summ of pay:\n");

            // Введение данных в консоль
            // std::cin>>Command;
            io::stdin().read_to_string(&mut command);

            decode_base10(sum,Command, btc_decimal_places);

            while (sum > MyBalance || sum == 0) {
                // Вывод данных на консоль
                std::cout<<"your in-channel balance is "<<MyBalance<<"please write summ less that it value:";
                // Ввод данных в консоль
                // std::cin>>command;
                io::stdin().read_to_string(&mut command);
                decode_base10(sum,Command, btc_decimal_places);
            }

            //дадим серверу знать о том, что мы хотим совершить платеж по каналу.
            socket->send(boost::asio::buffer(&message, sizeof(int8_t)));
            //передадим сумму которую желаем передать по каналу
            socket->send(boost::asio::buffer(&sum, sizeof(uint64_t)));

            //меняем балансы
            MyBalance -= sum;
            ServerBalance += sum;
            //отправим серверу секретное значение(per_commitment_secret), для старой транзакции
            socket->send(boost::asio::buffer(&per_commitment_secret, sizeof(ec_secret)));
            //генерируем новые per_commitment_secret и per_commitment_basepoint
            pseudo_random_fill(my_entropy);
            per_commitment_secret = to_array<ec_secret_size>(my_entropy);
            per_commitment_basepoint = gen_point;
            ec_multiply(per_commitment_basepoint, per_commitment_secret);
            //отправляем сгенерированную только что per_commitment_basepoint серверу
            socket->send(boost::asio::buffer(&per_commitment_basepoint, ec_compressed_size));

            //отправим серверу хэш от предыдущей транзакции обязательстве(которую отправлял сервер, а клинт подписал после предыдуще оплаты)
            let cur_hash: hash_digest = remote_commit_tx.hash();
            socket->send(boost::asio::buffer(&cur_hash, hash_size));
            //получаем от сервера его транзакцию обязательство (с обновленным балансом)
            let current_remote_commit_TX: transaction::Transaction = receive_transaction(socket);
            //std::cout<<"commit tx code for sending in blockchain:\n"<<encode_base16( current_remote_commit_TX.hash() );

            //подпишем полченную транзакцию обязательство
            let Sig0: endorsement;
            let sig_script0: operation::list;

            script::create_endorsement(Sig0, user_secret, funding_tx.outputs()[0].script(), current_remote_commit_TX,0,0x01);

            sig_script0.push_back(operation(opcode::push_size_0));
            sig_script0.push_back(current_remote_commit_TX.inputs()[0].script().operations()[0]);
            sig_script0.push_back(operation(Sig0));

            script InputScript0(sig_script0);
            current_remote_commit_TX.inputs()[0].set_script(InputScript0);

            //проверяем полученную транзакцию обязательство(соответсвие входа выходу образующей транзакции канала, и выходных балансов. а также то, что для выходов клиент действительно использовал переданную ему от сервера отдоразовую точку)
            //оставим проверку на потои, пока представим идеальную ситуацию в которой клиент не пытается обмануть сервер

            //в случае удачных проверок(если транзакция-обязательство полученная от сервера валидна) заменяем
            remote_commit_tx = current_remote_commit_TX;
            // Вывод данных на консольк
            std::cout<<"commit tx code for sending in blockchain:\n"<<encode_base16( remote_commit_tx.to_data() );
            std::cout<<"commit tx code for sending in blockchain:\n"<<encode_base16( remote_commit_tx.hash() );


            //отправляем серверу свою новую транзакцию обязательство
            commit_tx = create_commitment_tx(funding_tx,ServerPublicKey, user_secret, revocation_basepoint,remote_per_commitment_basepoint,MyBalance,ServerBalance);
            send_transaction(commit_tx, socket);

            //возможно нужно подождать от сервера ответ - принимает ли он платеж или нет
        }

        //просим сервер закрыть канал
        if (command == "-close") {
            message = 1;

            //дадим серверу знать о том, что мы хотим закрыть канал.
            socket->send(boost::asio::buffer(&message, sizeof(int8_t)));
            //закрывать канал будем по алгоритму описанному на
            //https://lightning.network/lightning-network-paper.pdf
            //стр 28
            //для начала создаем транзакцию
            let lastTX: transaction;

            //вход
            output_point UTXO(funding_tx.hash(), 0u); //вход транзакции, это первый(с нулевым индексом) выход образующей канал транзакции

            let input0: input;
            input0.set_previous_output(UTXO);
            input0.set_sequence(0xffffffff);

            lastTX.inputs().push_back(input0); //добавим вход без подписи

            //выходы
            let output0: output;  //выход на свой адрес с p2pkh выходом
            output0.set_value(MyBalance- (MyBalance * CLOSING_CHANNEL_FEES/(MyBalance+ServerBalance)));
            let p2pkhScript: operation::list = script::to_pay_key_hash_pattern(ClientPubKey.to_payment_address().hash());
            output0.set_script(p2pkhScript);

            let output1: output;
            output1.set_value(ServerBalance- (ServerBalance * CLOSING_CHANNEL_FEES/(MyBalance+ServerBalance)));
            let p2pkhScriptToServer: operation::list = script::to_pay_key_hash_pattern(ServerPublicKey.to_payment_address().hash());
            output1.set_script(p2pkhScriptToServer);

            lastTX.outputs().push_back(output0);
            lastTX.outputs().push_back(output1);
            //подпишем транзакцию своим приватным ключом

            let Sig0: endorsement;
            // считаем скрипт выхода соответствующнго входу0
            script::create_endorsement(Sig0, user_secret, funding_tx.outputs()[0].script(),  lastTX,0u,0x01);

            let sig_script0: operation::list;

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

fn main() {
    let mut command = String::new();

    let io_service: boost::asio::io_service;
    boost::asio::ip::tcp::endpoint Endpoint(boost::asio::ip::address_v4::from_string("127.0.0.1"), 17666); //localhost, порт 17666
    let socket: boost::asio::ip::tcp::socket = new boost::asio::ip::tcp::socket(IOservice);

    while (true) {

        println!("write \"-create_channel\" for creating channel or \"-exit\"\n");

        // Ввод данных в консоль
        io::stdin().read_to_string(&mut command);

        if (command =="-exit") {
            break;
        }
        if (command =="-create_channel") {
            socket->async_connect(Endpoint, std::bind( client_handler, socket, std::placeholders::_1) );
            io_service.run();
        }
    }
}