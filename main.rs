//zclimate
//John Bruhling
//@autotunafish
//12-14-22 22:40 UTC

use chrono::{TimeZone, Utc};
use inotify::{Inotify, WatchMask};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::{thread, time};

use std::fmt::Write as OtherWrite;
use std::process;

fn main() {
    //start by building the command prefixes for zcash-cli
    println!();
    println!("\x1b[48;5;58m\x1b[38;5;187mEnter path to zcash.conf\x1b[0m");
    println!("\x1b[48;5;54m\x1b[38;5;187m--example /home/ferris/.zcash/zcash.conf\x1b[0m");

    println!("\x1b[48;5;52m\x1b[38;5;187mInputting an invalid path is not recoverable!\x1b[0m");

    //THIS WILL PANIC IF INVALID
    //TODO: ADD CHECK FOR FILE EXISTENCE?
    println!("\x1b[48;5;52m\x1b[38;5;187m\x1b[0m");
    //confpathline is the line to the conf path
    let mut confpathline = String::new();
    //get input
    //BAD INPUT SHOULD FAIL so PANIC
    io::stdin().read_line(&mut confpathline).unwrap();
    //pop newline
    confpathline.pop();
    //println!("confpathline {}", confpathline);
    //p is all the conf contents as a String
    let p = fs::read_to_string(&confpathline).unwrap();

    //println!("contents {}", p);
    //bools to indicate if the conf contents has any rpc options
    let mut rpc = false;
    let mut pswd = false;
    let mut auth = false;
    //pvc is a place holder for the indv conf lines
    let mut pvc;
    let mut kvc;

    //these are the rpc option Strings used for the commands, if any
    let mut theuser = String::new();
    let mut thepswrd = String::new();
    let mut thecookie = String::new();
    //check for the existence of all and set the bools
    if p.contains("rpcpassword") {
        pswd = true;
    }
    //println!("found rpcpassword {:?} at {:?}", &pswd, &p.find("rpcpassword"));
    if p.contains("rpcuser") {
        rpc = true;
    }
    //println!("found rpcuser {:?} at {:?}", &rpc, &p.find("rpcuser"));
    if p.contains("rpcauth") {
        auth = true;
    }
    //println!("found rpcauth {:?} at {:?}", &auth, &p.find("rpcauth"));
    //split the conf into lines, if any
    let mut plines = p.split_whitespace();
    //iterate over all the lines, if any
    loop {
        //get first line, if any
        let pv = plines.next().clone();
        //assert EQ for Some
        if pv != None {
            //remove Result wrapper
            pvc = pv.unwrap();
            //println!("pvc {:?}", &pvc);
            //match to contents and set rpc Strings if true
            if pswd == true {
                if pvc.contains("rpcpassword") {
                    thepswrd = pvc.to_string().clone();
                }
            }
            if rpc == true {
                if pvc.contains("rpcuser") {
                    theuser = pvc.to_string().clone();
                }
            }
            if auth == true {
                if pvc.contains("rpcauth") {
                    thecookie = pvc.to_string().clone();
                }
            }
            continue;
        }
        break;
    }
    //else set the rpc Strings to empty if false
    if pswd == false {
        thepswrd.push_str("");
    }
    if rpc == false {
        theuser.push_str("");
    }
    if auth == false {
        thecookie.push_str("");
    }
    //println!("thepswrd {}", &thepswrd);
    //println!("theuser {}", &theuser);
    //println!("thecookie {}", &thecookie);
    //get path to the zcash/src folder
    println!();
    println!("\x1b[48;5;58m\x1b[38;5;187mEnter path to zcash/src\x1b[0m");
    println!("\x1b[48;5;54m\x1b[38;5;187m--example   /home/ferris/zcash/src\x1b[0m");
    println!("\x1b[48;5;52m\x1b[38;5;187mInputting an invalid path is not recoverable!\x1b[0m");

    //THIS WILL PANIC IF INVALID BUT MUST FAIL IF INCORRECT!
    //TODO: ADD CHECK FOR FILE EXISTENCE?
    println!("\x1b[48;5;52m\x1b[38;5;187m\x1b[0m");
    let mut srcpathline = String::new();
    //get input
    io::stdin().read_line(&mut srcpathline).unwrap();
    //pop newline
    srcpathline.pop();
    //format length, end '/' not necessary
    let slash = srcpathline.pop().unwrap();
    if slash != '/' {
        srcpathline.push(slash);
    }
    //println!("srcpathline {}", srcpathline);

    println!();
    println!("\x1b[48;5;58m\x1b[38;5;187mEnter path to .zcash/debug.log \x1b[0m");
    println!("\x1b[48;5;54m\x1b[38;5;187m--example   /home/ferris/.zcash/debug.log\x1b[0m");
    println!("\x1b[48;5;52m\x1b[38;5;187mInputting an invalid path is not recoverable!\x1b[0m");

    println!();
    let mut dblpathline = String::new();
    //get input
    io::stdin().read_line(&mut dblpathline).unwrap();
    //pop newline
    dblpathline.pop();
    //format length, end '/' not necessary
    let slash = dblpathline.pop().unwrap();
    if slash != '/' {
        dblpathline.push(slash);
    }

    //begin building Strings for the options and command to send
    //first part
    let resend = String::from("./zcash-cli -conf=");
    //second part
    let s3 = String::from(&confpathline);
    //third optional rpc part, will be "" if else
    let mut s4 = String::from("");
    //push a space and a dash followed by the option
    if pswd == true {
        s4.push_str(" -");
        s4.push_str(&thepswrd.as_str());
    }
    if rpc == true {
        s4.push_str(" -");
        s4.push_str(&theuser.as_str());
    }
    if auth == true {
        s4.push_str(" -");
        s4.push_str(&thecookie.as_str());
    }
    //println!("s4 {}", &s4);
    //fourth part, output file receives both stdout and stderr
    let s6 = String::from("> ~/zcash/src/climateuse0.txt 2>&1");
    let s7 = String::from("> ~/zcash/src/climateuse1.txt 2>&1");
    let s8 = String::from("> ~/zcash/src/climateuse2.txt 2>&1");

    //pre-format it all for push_str method below
    let resend00 = s3.as_str();
    let resend000 = s4.as_str();
    let resend00000 = s6.as_str();
    let resend00000x = s7.as_str();
    let resend00000y = s8.as_str();

    //first part of building 3 identical Strings for the file paths
    let mut sp = String::from(&srcpathline);
    let mut sp1 = sp.clone();
    let mut sp2 = sp.clone();
    let mut rp = sp.clone();
    let mut rp1 = sp.clone();
    let mut rp2 = sp.clone();
    let mut tp = sp.clone();
    let mut tp1 = sp.clone();
    let mut tp2 = sp.clone();

    //second unique part for the three files
    let sptail = String::from("/climateusenox.txt");
    sp.push_str(&sptail);
    //println!("sp {}", sp);
    let sp1tail = String::from("/climateusenox1.txt");
    sp1.push_str(&sp1tail);
    //println!("sp1 {}", sp1);
    let sp2tail = String::from("/climateuse0.txt");
    sp2.push_str(&sp2tail);
    //println!("sp2 {}", sp2);
    let rptail = String::from("/climateusenox2.txt");
    rp.push_str(&rptail);
    //println!("rp {}", rp);
    let rp1tail = String::from("/climateusenox3.txt");
    rp1.push_str(&rp1tail);
    //println!("rp1 {}", rp1);
    let rp2tail = String::from("/climateuse1.txt");
    rp2.push_str(&rp2tail);
    //println!("rp2 {}", rp2);
    let tptail = String::from("/climateusenox4.txt");
    tp.push_str(&tptail);
    //println!("tp {}", tp);
    let tp1tail = String::from("/climateusenox5.txt");
    tp1.push_str(&tp1tail);
    //println!("tp1 {}", tp1);
    let tp2tail = String::from("/climateuse2.txt");
    tp2.push_str(&tp2tail);
    //println!("tp2 {}", tp2);

    //main loop
    loop {
        //Select an Option
        println!();
println!("\x1b[48;5;54m\x1b[38;5;187mType [] to select:\nG - getinfo | L - Show RX History | B - Wallet | A - All Addresses | N - New Account | F - New Unified Address | H - Get All TXs | C - z_getbalance | O - Operations | S - Sign Message | V - Verify message | X - Exit\x1b[0m");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            //Every match goes about the same except some will take more input,
            //or have nested calls; these require different event handlers.
            "O" => {
                {
                    //Create the command by pushing the above input //'resend####' values.
                    let s5 = String::from(" z_getoperationstatus ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    //Write to file where ./climatenotify# is watching.
                    //It gets cp'd to another file, executed and the
                    //output is captured below by the inotify event
                    //and taken into an iterator. It's matched and
                    //values printed and/or the value passed to the next
                    //call.
                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    let mut klines = k.split_whitespace();
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();
                            match kvc {
                                "[" | "]" | "{" | "}" | "\\" | "}," | "]," | "\"addresses\":"
                                | "\"source\":" => {}
                                "\"seedfp\":" | "\"diversifier_index\":" => {
                                    klines.next();
                                }
                                _ => println!(
                                    "\x1b[48;5;53m\x1b[38;5;187m{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap()
                                ),
                            }
                            if kvc.len() >= 24 {
                                println!();
                            }
                            continue;
                        }
                        break;
                    }
                }
            }

            //runs getinfo
            "G" => {
                {
                    //println!("G - getinfo ok!");

                    let s5 = String::from(" getinfo ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    let mut klines = k.split_whitespace();
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();
                            match kvc {
                                "[" | "]" | "{" | "}" | "\\" | "}," | "]," | "\"addresses\":"
                                | "\"source\":" => {}
                                "\"seedfp\":" | "\"diversifier_index\":" => {
                                    klines.next();
                                }
                                _ => println!(
                                    "\x1b[48;5;53m\x1b[38;5;187m{}{}\x1b[0m",
                                    &kvc,
                                    &klines.next().unwrap()
                                ),
                            }
                            if kvc.len() >= 24 {
                                println!();
                            }
                            continue;
                        }
                        break;
                    }
                }
            }

            "H" => {
                println!("\x1b[48;5;52m\x1b[38;5;187m ! This May Take a Few Minutes, Procede Anyways?\x1b[0m");
                println!("\x1b[48;5;58m\x1b[38;5;187mEnter Y to Continue or Enter Any Other Input to Return\x1b[0m");

                let mut zadd = String::from(&dblpathline.clone());

                io::stdin().read_line(&mut zadd).unwrap();
                zadd.pop();
                let yes = "Y";

                let mut zaddy = zadd.replace('"', "");
                //println!("zaddy   {}", &zaddy);
                //println!("zaddybg {:?}", &zaddy);

                if zaddy.contains(&yes) == true {
                    zaddy.clear();

                    let p = String::from("/home/john/.zcash/debug.log");

                    println!(
                        "\x1b[48;5;58m\x1b[38;5;187mFetching TX History, Please Wait...\x1b[0m"
                    );

                    println!();

                    //println!("{}", &p);
                    let mut list = String::new();
                    //let mut counter = 0;
                    let k = fs::read_to_string(&p).unwrap();
                    let mut klines = k.split_whitespace();
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some

                        if kv != None {
                            //remove Result wrapper
                            let kvc = kv.unwrap();

                            if kvc.contains("AddToWallet") == true {
                                let clinn = klines.next().clone();
                                let linn = clinn.unwrap();
                                //println!("{}", &linn);

                                if list.contains(&linn) != true {
                                    list.push_str(linn);
                                    list.push(' ');
                                }
                            }

                            continue;
                        }

                        break;
                    }
                    //println!("{:?}", &list);
                    //println!("Found {} TXID's", &list.len());

                    let mut counter = 0;
                    klines = list.split_whitespace();
                    loop {
                        //get first line, if
                        counter = counter + 1;
                        if counter % 20 == 0 {
                            println!(
                                "\x1b[48;5;52m\x1b[38;5;187mPress Enter to List Next 20\x1b[0m"
                            );

                            let mut input = String::new();

                            io::stdin().read_line(&mut input).unwrap();
                        }

                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            let mut kvc = kv.unwrap();
                            //println!("{:?}", &kvc);
                            let faddy = String::from(kvc.clone());

                            println!();
                            println!("\x1b[48;5;32m\x1b[38;5;187m #: {}\x1b[0m", &counter);

                            {
                                {
                                    //println!("C - z_getbalance ok!");

                                    let s5 = String::from(" gettransaction ");
                                    //println!("Paste in your sapling address below");

                                    let mut zadd = String::from(kvc).clone();

                                    //io::stdin().read_line(&mut zadd).unwrap();
                                    //zadd.pop();
                                    zadd.push(' ');

                                    let zaddy = zadd.replace('"', "");
                                    //println!("zaddy   {}", &zaddy);
                                    //println!("zaddybg {:?}", &zaddy);

                                    let mut resend0 = resend.clone();
                                    let resend0000 = s5.as_str();

                                    resend0.push_str(resend00);
                                    resend0.push_str(resend000);
                                    resend0.push_str(resend0000);
                                    resend0.push_str(zaddy.as_str());
                                    resend0.push_str(resend00000x);

                                    //println!("{}\n", &resend0);

                                    let mut var_1 = File::create(&rp).expect("no");
                                    var_1.write_all(&resend0.as_bytes()).expect("no");
                                }

                                let mut inotify = Inotify::init()
                                    .expect("Error while initializing inotify instance");
                                inotify
                                    .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                    .expect("Failed to add file watch");

                                let mut buffer = [0; 512];
                                let events = inotify
                                    .read_events_blocking(&mut buffer)
                                    .expect("Error while reading events");
                                #[allow(unused_variables)]
                                for event in events {
                                    let k = fs::read_to_string(&rp2).unwrap();
                                    let mut klines = k.split_whitespace();
                                    loop {
                                        //get first line, if any
                                        let kv = klines.next().clone();
                                        //assert EQ for Some
                                        if kv != None {
                                            //remove Result wrapper
                                            kvc = kv.unwrap();

                                            match kvc {
                                                "\"time\":" => {
                                                    let timeget = klines.next();
                                                    if timeget != None {
                                                        //println!("{:?}", &timeget);
                                                        let mut gottime =
                                                            timeget.clone().expect("").to_string();
                                                        gottime.pop();
                                                        let goime = gottime.parse().expect("");
                                                        //println!("{:?}", &goime);

                                                        #[allow(deprecated)]
                                                        let dt = Utc.timestamp(goime, 0);
                                                        println!();
                                                        println!("\x1b[48;5;59m\x1b[38;5;187m{}{}\x1b[0m", &kvc, &dt);
                                                    }
                                                }

                                                _ => (),
                                            }

                                            continue;
                                        }
                                        break;
                                    }

                                    let ten_millis = time::Duration::from_millis(50);
                                    let now = time::Instant::now();

                                    thread::sleep(ten_millis);

                                    //println!("\x1b[48;5;23m\x1b[38;5;187m{:?}\x1b[0m", &k);
                                }
                            }

                            {
                                {
                                    //println!("C - z_getbalance ok!");

                                    let s5 = String::from(" z_viewtransaction ");
                                    //println!("Paste in your sapling address below");

                                    let mut zadd = String::from(faddy).clone();

                                    zadd.push(' ');

                                    let zaddy = zadd.replace('"', "");
                                    //println!("zaddy   {}", &zaddy);
                                    //println!("zaddybg {:?}", &zaddy);

                                    let mut resend0 = resend.clone();
                                    let resend0000 = s5.as_str();

                                    resend0.push_str(resend00);
                                    resend0.push_str(resend000);
                                    resend0.push_str(resend0000);
                                    resend0.push_str(zaddy.as_str());
                                    resend0.push_str(resend00000y);

                                    //println!("{}\n", &resend0);

                                    let mut var_1 = File::create(&tp).expect("no");
                                    var_1.write_all(&resend0.as_bytes()).expect("no");
                                }

                                let mut inotify = Inotify::init()
                                    .expect("Error while initializing inotify instance");
                                inotify
                                    .add_watch(&tp2, WatchMask::CLOSE_WRITE)
                                    .expect("Failed to add file watch");

                                let mut buffer = [0; 512];
                                let events = inotify
                                    .read_events_blocking(&mut buffer)
                                    .expect("Error while reading events");
                                #[allow(unused_variables)]
                                for event in events {
                                    let k = fs::read_to_string(&tp2).unwrap();
                                    let mut klines = k.split_whitespace();
                                    loop {
                                        //get first line, if any
                                        let kv = klines.next().clone();
                                        //assert EQ for Some
                                        if kv != None {
                                            //remove Result wrapper
                                            kvc = kv.unwrap();

                                            if kvc.len() >= 400 {
                                                let mut h = String::from(kvc.clone());
                                                h = h.replace("\"", "");

                                                loop {
                                                    if h.len() == 0 {
                                                        break;
                                                    }
                                                    let mut x = String::new();
                                                    loop {
                                                        if h.len() == 0 {
                                                            break;
                                                        }
                                                        let y = h.remove(0).to_ascii_lowercase();
                                                        if y == '\u{20}' {
                                                            continue;
                                                        }
                                                        x.push(y);
                                                        break;
                                                    }
                                                    loop {
                                                        if h.len() == 0 {
                                                            break;
                                                        }
                                                        let z = h.remove(0).to_ascii_lowercase();
                                                        if z == '\u{20}' {
                                                            continue;
                                                        }
                                                        x.push(z);
                                                        break;
                                                    }
                                                    match x.as_str() {
                                                        //decoded output
                                                        "00" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{00}\x1b[0m");
                                                        }
                                                        "01" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{01}\x1b[0m");
                                                        }
                                                        "02" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{02}\x1b[0m");
                                                        }
                                                        "03" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{03}\x1b[0m");
                                                        }
                                                        "04" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{04}\x1b[0m");
                                                        }
                                                        "05" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{05}\x1b[0m");
                                                        }
                                                        "06" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{06}\x1b[0m");
                                                        }
                                                        "07" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{07}\x1b[0m");
                                                        }
                                                        "08" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{08}\x1b[0m");
                                                        }
                                                        "09" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{09}\x1b[0m");
                                                        }
                                                        "0a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0a}\x1b[0m");
                                                        }
                                                        "0b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0b}\x1b[0m");
                                                        }
                                                        "0c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0c}\x1b[0m");
                                                        }
                                                        "0d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0d}\x1b[0m");
                                                        }
                                                        "0e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0e}\x1b[0m");
                                                        }
                                                        "0f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0f}\x1b[0m");
                                                        }
                                                        "10" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{10}\x1b[0m");
                                                        }
                                                        "11" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{11}\x1b[0m");
                                                        }
                                                        "12" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{12}\x1b[0m");
                                                        }
                                                        "13" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{13}\x1b[0m");
                                                        }
                                                        "14" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{14}\x1b[0m");
                                                        }
                                                        "15" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{15}\x1b[0m");
                                                        }
                                                        "16" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{16}\x1b[0m");
                                                        }
                                                        "17" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{17}\x1b[0m");
                                                        }
                                                        "18" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{18}\x1b[0m");
                                                        }
                                                        "19" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{19}\x1b[0m");
                                                        }
                                                        "1a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{1a}\x1b[0m");
                                                        }
                                                        "1b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{1b}\x1b[0m");
                                                        }
                                                        "1c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{1c}\x1b[0m");
                                                        }
                                                        "1d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{1d}\x1b[0m");
                                                        }
                                                        "1e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{1e}\x1b[0m");
                                                        }
                                                        "1f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{1f}\x1b[0m");
                                                        }
                                                        "20" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{20}\x1b[0m");
                                                        }
                                                        "21" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{21}\x1b[0m");
                                                        }
                                                        "22" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{22}\x1b[0m");
                                                        }
                                                        "23" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{23}\x1b[0m");
                                                        }
                                                        "24" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{24}\x1b[0m");
                                                        }
                                                        "25" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{25}\x1b[0m");
                                                        }
                                                        "26" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{26}\x1b[0m");
                                                        }
                                                        "27" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{27}\x1b[0m");
                                                        }
                                                        "28" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{28}\x1b[0m");
                                                        }
                                                        "29" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{29}\x1b[0m");
                                                        }
                                                        "2a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{2a}\x1b[0m");
                                                        }
                                                        "2b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{2b}\x1b[0m");
                                                        }
                                                        "2c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{2c}\x1b[0m");
                                                        }
                                                        "2d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{2d}\x1b[0m");
                                                        }
                                                        "2e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{2e}\x1b[0m");
                                                        }
                                                        "2f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{2f}\x1b[0m");
                                                        }
                                                        "30" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{30}\x1b[0m");
                                                        }
                                                        "31" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{31}\x1b[0m");
                                                        }
                                                        "32" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{32}\x1b[0m");
                                                        }
                                                        "33" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{33}\x1b[0m");
                                                        }
                                                        "34" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{34}\x1b[0m");
                                                        }
                                                        "35" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{35}\x1b[0m");
                                                        }
                                                        "36" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{36}\x1b[0m");
                                                        }
                                                        "37" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{37}\x1b[0m");
                                                        }
                                                        "38" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{38}\x1b[0m");
                                                        }
                                                        "39" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{39}\x1b[0m");
                                                        }
                                                        "3a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{3a}\x1b[0m");
                                                        }
                                                        "3b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{3b}\x1b[0m");
                                                        }
                                                        "3c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{3c}\x1b[0m");
                                                        }
                                                        "3d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{3d}\x1b[0m");
                                                        }
                                                        "3e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{3e}\x1b[0m");
                                                        }
                                                        "3f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{3f}\x1b[0m");
                                                        }
                                                        "40" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{40}\x1b[0m");
                                                        }
                                                        "41" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{41}\x1b[0m");
                                                        }
                                                        "42" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{42}\x1b[0m");
                                                        }
                                                        "43" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{43}\x1b[0m");
                                                        }
                                                        "44" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{44}\x1b[0m");
                                                        }
                                                        "45" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{45}\x1b[0m");
                                                        }
                                                        "46" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{46}\x1b[0m");
                                                        }
                                                        "47" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{47}\x1b[0m");
                                                        }
                                                        "48" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{48}\x1b[0m");
                                                        }
                                                        "49" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{49}\x1b[0m");
                                                        }
                                                        "4a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{4a}\x1b[0m");
                                                        }
                                                        "4b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{4b}\x1b[0m");
                                                        }
                                                        "4c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{4c}\x1b[0m");
                                                        }
                                                        "4d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{4d}\x1b[0m");
                                                        }
                                                        "4e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{4e}\x1b[0m");
                                                        }
                                                        "4f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{4f}\x1b[0m");
                                                        }
                                                        "50" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{50}\x1b[0m");
                                                        }
                                                        "51" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{51}\x1b[0m");
                                                        }
                                                        "52" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{52}\x1b[0m");
                                                        }
                                                        "53" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{53}\x1b[0m");
                                                        }
                                                        "54" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{54}\x1b[0m");
                                                        }
                                                        "55" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{55}\x1b[0m");
                                                        }
                                                        "56" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{56}\x1b[0m");
                                                        }
                                                        "57" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{57}\x1b[0m");
                                                        }
                                                        "58" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{58}\x1b[0m");
                                                        }
                                                        "59" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{59}\x1b[0m");
                                                        }
                                                        "5a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{5a}\x1b[0m");
                                                        }
                                                        "5b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{5b}\x1b[0m");
                                                        }
                                                        "5c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{5c}\x1b[0m");
                                                        }
                                                        "5d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{5d}\x1b[0m");
                                                        }
                                                        "5e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{5e}\x1b[0m");
                                                        }
                                                        "5f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{5f}\x1b[0m");
                                                        }
                                                        "60" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{60}\x1b[0m");
                                                        }
                                                        "61" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{61}\x1b[0m");
                                                        }
                                                        "62" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{62}\x1b[0m");
                                                        }
                                                        "63" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{63}\x1b[0m");
                                                        }
                                                        "64" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{64}\x1b[0m");
                                                        }
                                                        "65" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{65}\x1b[0m");
                                                        }
                                                        "66" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{66}\x1b[0m");
                                                        }
                                                        "67" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{67}\x1b[0m");
                                                        }
                                                        "68" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{68}\x1b[0m");
                                                        }
                                                        "69" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{69}\x1b[0m");
                                                        }
                                                        "6a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{6a}\x1b[0m");
                                                        }
                                                        "6b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{6b}\x1b[0m");
                                                        }
                                                        "6c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{6c}\x1b[0m");
                                                        }
                                                        "6d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{6d}\x1b[0m");
                                                        }
                                                        "6e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{6e}\x1b[0m");
                                                        }
                                                        "6f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{6f}\x1b[0m");
                                                        }
                                                        "70" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{70}\x1b[0m");
                                                        }
                                                        "71" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{71}\x1b[0m");
                                                        }
                                                        "72" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{72}\x1b[0m");
                                                        }
                                                        "73" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{73}\x1b[0m");
                                                        }
                                                        "74" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{74}\x1b[0m");
                                                        }
                                                        "75" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{75}\x1b[0m");
                                                        }
                                                        "76" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{76}\x1b[0m");
                                                        }
                                                        "77" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{77}\x1b[0m");
                                                        }
                                                        "78" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{78}\x1b[0m");
                                                        }
                                                        "79" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{79}\x1b[0m");
                                                        }
                                                        "7a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{7a}\x1b[0m");
                                                        }
                                                        "7b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{7b}\u{7b}\x1b[0m");
                                                        }
                                                        "7c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{7c}\x1b[0m");
                                                        }
                                                        "7d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{7d}\u{7d}\x1b[0m");
                                                        }
                                                        "7e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{7e}\x1b[0m");
                                                        }
                                                        "7f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{7f}\x1b[0m");
                                                        }
                                                        "80" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{80}\x1b[0m");
                                                        }
                                                        "81" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0081}\x1b[0m");
                                                        }
                                                        "82" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0082}\x1b[0m");
                                                        }
                                                        "83" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0083}\x1b[0m");
                                                        }
                                                        "84" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0084}\x1b[0m");
                                                        }
                                                        "85" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0085}\x1b[0m");
                                                        }
                                                        "86" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0086}\x1b[0m");
                                                        }
                                                        "87" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0087}\x1b[0m");
                                                        }
                                                        "88" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0088}\x1b[0m");
                                                        }
                                                        "89" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{0089}\x1b[0m");
                                                        }
                                                        "8a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{008a}\x1b[0m");
                                                        }
                                                        "8b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{008b}\x1b[0m");
                                                        }
                                                        "8c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{8c}\x1b[0m");
                                                        }
                                                        "8d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{8d}\x1b[0m");
                                                        }
                                                        "8e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{8e}\x1b[0m");
                                                        }
                                                        "8f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{8f}\x1b[0m");
                                                        }
                                                        "90" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{90}\x1b[0m");
                                                        }
                                                        "91" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{91}\x1b[0m");
                                                        }
                                                        "92" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{92}\x1b[0m");
                                                        }
                                                        "93" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{93}\x1b[0m");
                                                        }
                                                        "94" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{94}\x1b[0m");
                                                        }
                                                        "95" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{95}\x1b[0m");
                                                        }
                                                        "96" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{96}\x1b[0m");
                                                        }
                                                        "97" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{97}\x1b[0m");
                                                        }
                                                        "98" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{98}\x1b[0m");
                                                        }
                                                        "99" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{99}\x1b[0m");
                                                        }
                                                        "9a" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{9a}\x1b[0m");
                                                        }
                                                        "9b" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{9b}\x1b[0m");
                                                        }
                                                        "9c" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{9c}\x1b[0m");
                                                        }
                                                        "9d" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{9d}\x1b[0m");
                                                        }
                                                        "9e" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{9e}\x1b[0m");
                                                        }
                                                        "9f" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{9f}\x1b[0m");
                                                        }
                                                        "a0" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a0}\x1b[0m");
                                                        }
                                                        "a1" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a1}\x1b[0m");
                                                        }
                                                        "a2" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a2}\x1b[0m");
                                                        }
                                                        "a3" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a3}\x1b[0m");
                                                        }
                                                        "a4" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a4}\x1b[0m");
                                                        }
                                                        "a5" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a5}\x1b[0m");
                                                        }
                                                        "a6" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a6}\x1b[0m");
                                                        }
                                                        "a7" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a7}\x1b[0m");
                                                        }
                                                        "a8" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a8}\x1b[0m");
                                                        }
                                                        "a9" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{a9}\x1b[0m");
                                                        }
                                                        "aa" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{aa}\x1b[0m");
                                                        }
                                                        "ab" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ab}\x1b[0m");
                                                        }
                                                        "ac" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ac}\x1b[0m");
                                                        }
                                                        "ad" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ad}\x1b[0m");
                                                        }
                                                        "ae" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ae}\x1b[0m");
                                                        }
                                                        "af" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{af}\x1b[0m");
                                                        }
                                                        "b0" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b0}\x1b[0m");
                                                        }
                                                        "b1" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b1}\x1b[0m");
                                                        }
                                                        "b2" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b2}\x1b[0m");
                                                        }
                                                        "b3" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b3}\x1b[0m");
                                                        }
                                                        "b4" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b4}\x1b[0m");
                                                        }
                                                        "b5" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b5}\x1b[0m");
                                                        }
                                                        "b6" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b6}\x1b[0m");
                                                        }
                                                        "b7" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b7}\x1b[0m");
                                                        }
                                                        "b8" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b8}\x1b[0m");
                                                        }
                                                        "b9" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{b9}\x1b[0m");
                                                        }
                                                        "ba" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ba}\x1b[0m");
                                                        }
                                                        "bb" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{bb}\x1b[0m");
                                                        }
                                                        "bc" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{bc}\x1b[0m");
                                                        }
                                                        "bd" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{bd}\x1b[0m");
                                                        }
                                                        "be" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{be}\x1b[0m");
                                                        }
                                                        "bf" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{bf}\x1b[0m");
                                                        }
                                                        "c0" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c0}\x1b[0m");
                                                        }
                                                        "c1" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c1}\x1b[0m");
                                                        }
                                                        "c2" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c2}\x1b[0m");
                                                        }
                                                        "c3" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c3}\x1b[0m");
                                                        }
                                                        "c4" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c4}\x1b[0m");
                                                        }
                                                        "c5" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c5}\x1b[0m");
                                                        }
                                                        "c6" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c6}\x1b[0m");
                                                        }
                                                        "c7" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c7}\x1b[0m");
                                                        }
                                                        "c8" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c8}\x1b[0m");
                                                        }
                                                        "c9" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{c9}\x1b[0m");
                                                        }
                                                        "ca" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ca}\x1b[0m");
                                                        }
                                                        "cb" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{cb}\x1b[0m");
                                                        }
                                                        "cc" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{cc}\x1b[0m");
                                                        }
                                                        "cd" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{cd}\x1b[0m");
                                                        }
                                                        "ce" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ce}\x1b[0m");
                                                        }
                                                        "cf" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{cf}\x1b[0m");
                                                        }
                                                        "d0" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d0}\x1b[0m");
                                                        }
                                                        "d1" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d1}\x1b[0m");
                                                        }
                                                        "d2" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d2}\x1b[0m");
                                                        }
                                                        "d3" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d3}\x1b[0m");
                                                        }
                                                        "d4" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d4}\x1b[0m");
                                                        }
                                                        "d5" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d5}\x1b[0m");
                                                        }
                                                        "d6" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d6}\x1b[0m");
                                                        }
                                                        "d7" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d7}\x1b[0m");
                                                        }
                                                        "d8" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d8}\x1b[0m");
                                                        }
                                                        "d9" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{d9}\x1b[0m");
                                                        }
                                                        "da" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{da}\x1b[0m");
                                                        }
                                                        "db" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{db}\x1b[0m");
                                                        }
                                                        "dc" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{dc}\x1b[0m");
                                                        }
                                                        "dd" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{dd}\x1b[0m");
                                                        }
                                                        "de" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{de}\x1b[0m");
                                                        }
                                                        "df" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{df}\x1b[0m");
                                                        }
                                                        "e0" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e0}\x1b[0m");
                                                        }
                                                        "e1" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e1}\x1b[0m");
                                                        }
                                                        "e2" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e2}\x1b[0m");
                                                        }
                                                        "e3" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e3}\x1b[0m");
                                                        }
                                                        "e4" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e4}\x1b[0m");
                                                        }
                                                        "e5" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e5}\x1b[0m");
                                                        }
                                                        "e6" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e6}\x1b[0m");
                                                        }
                                                        "e7" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e7}\x1b[0m");
                                                        }
                                                        "e8" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e8}\x1b[0m");
                                                        }
                                                        "e9" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{e9}\x1b[0m");
                                                        }
                                                        "ea" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ea}\x1b[0m");
                                                        }
                                                        "eb" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{eb}\x1b[0m");
                                                        }
                                                        "ec" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ec}\x1b[0m");
                                                        }
                                                        "ed" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ed}\x1b[0m");
                                                        }
                                                        "ee" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ee}\x1b[0m");
                                                        }
                                                        "ef" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ef}\x1b[0m");
                                                        }
                                                        "f0" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f0}\x1b[0m");
                                                        }
                                                        "f1" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f1}\x1b[0m");
                                                        }
                                                        "f2" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f2}\x1b[0m");
                                                        }
                                                        "f3" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f3}\x1b[0m");
                                                        }
                                                        "f4" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f4}\x1b[0m");
                                                        }
                                                        "f5" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f5}\x1b[0m");
                                                        }
                                                        "f6" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f6}\x1b[0m");
                                                        }
                                                        "f7" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f7}\x1b[0m");
                                                        }
                                                        "f8" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f8}\x1b[0m");
                                                        }
                                                        "f9" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{f9}\x1b[0m");
                                                        }
                                                        "fa" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{fa}\x1b[0m");
                                                        }
                                                        "fb" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{fb}\x1b[0m");
                                                        }
                                                        "fc" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{fc}\x1b[0m");
                                                        }
                                                        "fd" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{fd}\x1b[0m");
                                                        }
                                                        "fe" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{fe}\x1b[0m");
                                                        }
                                                        "ff" => {
                                                            print!("\x1b[48;5;57m\x1b[38;5;188m\u{ff}\x1b[0m");
                                                        }
                                                        _ => (),
                                                    }
                                                    x.clear();
                                                }
                                                println!();
                                                continue;
                                            }

                                            match kvc {
                                                "[" | "]" | "{" | "}" | "\\" | "}," | "],"
                                                | "\"addresses\":" | "\"source\":" => {}

                                                "\"seedfp\":"
                                                | "\"diversifier_index\":"
                                                | "\"valueZat\":"
                                                | "\"walletInternal\":"
                                                | "\"outgoing\":"
                                                | "\"outputPrev\":"
                                                | "\"txidPrev\":"
                                                | "\"memoStr\":"
                                                | "\"spends\":"
                                                | "\"outputs\":"
                                                | "\"pool\":" => {
                                                    klines.next();
                                                }

                                                "\"txid\":" => {
                                                    println!(
                                                        "\x1b[48;5;53m\x1b[38;5;187m{}{}\x1b[0m",
                                                        &kvc,
                                                        &klines.next().unwrap()
                                                    )
                                                }

                                                "\"address\":" => {
                                                    println!(
                                                        "\x1b[48;5;52m\x1b[38;5;187m{}{}\x1b[0m",
                                                        &kvc,
                                                        &klines.next().unwrap()
                                                    )
                                                }

                                                "\"value\":" => {
                                                    println!(
                                                        "\x1b[48;5;23m\x1b[38;5;187m{}{}\x1b[0m",
                                                        &kvc,
                                                        &klines.next().unwrap()
                                                    )
                                                }

                                                "\"account\":" | "\"receiver_types\":" => {
                                                    println!(
                                                        "\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m",
                                                        &kvc
                                                    )
                                                }

                                                "\"memo\":" => {
                                                    println!(
                                                        "\x1b[48;5;59m\x1b[38;5;187m{}\x1b[0m",
                                                        &kvc
                                                    );
                                                }

                                                "\"output\":" => {
                                                    println!("\x1b[48;5;59m\x1b[38;5;187mReceiver Address <-- \x1b[0m");
                                                }

                                                "\"spend\":" => {
                                                    let zerochk = klines.next().unwrap();
                                                    if zerochk == "0," {
                                                        println!("\x1b[48;5;58m\x1b[38;5;187mTransaction Type: Send \x1b[0m");
                                                        println!("\x1b[48;5;59m\x1b[38;5;187mSender Address --> \x1b[0m");
                                                    }
                                                }
                                                _ => (),
                                            }

                                            continue;
                                        }
                                        break;
                                    }

                                    let ten_millis = time::Duration::from_millis(50);
                                    let now = time::Instant::now();

                                    thread::sleep(ten_millis);

                                    //println!("\x1b[48;5;23m\x1b[38;5;187m{:?}\x1b[0m", &k);
                                }
                            }

                            continue;
                        }
                        break;
                    }
                }
            }

            "L" => {
                {
                    let s5 = String::from(" listaddresses ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    println!("\x1b[48;5;58m\x1b[38;5;188mEnter Number to Select\x1b[0m");

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut addynum = String::new();

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    let mut klines = k.split_whitespace();
                    let mut counter = 1;
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();

                            if kvc.len() >= 24 {
                                if kvc.starts_with("\"t1") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                    continue;
                                }
                                if kvc.starts_with("\"zs") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                    continue;
                                }
                                if kvc.starts_with("\"u1") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    counter = counter + 1;
                                    addynum.push(' ');
                                    continue;
                                }
                            }

                            continue;
                        }
                        break;
                    }

                    let zaddynum = addynum.replace(',', "");
                    //println!("{}", &zaddynum);
                    println!("\x1b[48;5;58m\x1b[38;5;188mEnter Number to Select\x1b[0m");

                    let mut input = String::new();

                    io::stdin().read_line(&mut input).unwrap();
                    //Take input and match to the zaddynum
                    //with klines
                    //println!("{:?}", &input);
                    input.pop();

                    let mut klines = zaddynum.split_whitespace();
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();
                            //match here

                            if input == kvc {
                                {
                                    {
                                        let kv = klines.next().clone();
                                        let s5 = String::from(" z_listreceivedbyaddress ");
                                        let mut resend0 = resend.clone();
                                        let resend0000 = s5.as_str();

                                        resend0.push_str(resend00);
                                        resend0.push_str(resend000);
                                        resend0.push_str(resend0000);
                                        resend0.push_str(&kv.unwrap());
                                        resend0.push(' ');
                                        resend0.push_str(resend00000);

                                        //println!("{}\n", &resend0);

                                        let mut var_1 = File::create(&sp).expect("no");
                                        var_1.write_all(&resend0.as_bytes()).expect("no");
                                    }

                                    let mut inotify = Inotify::init()
                                        .expect("Error while initializing inotify instance");
                                    inotify
                                        .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                                        .expect("Failed to add file watch");

                                    let mut buffer = [0; 512];
                                    let events = inotify
                                        .read_events_blocking(&mut buffer)
                                        .expect("Error while reading events");
                                    #[allow(unused_variables)]
                                    for event in events {
                                        let k = fs::read_to_string(&sp2).unwrap();
                                        //TODO ERROR HANDLE FOR BAD INPUT, WILL PANIC
                                        let mut klines = k.split_whitespace();
                                        loop {
                                            //get first line, if any
                                            let kv = klines.next().clone();
                                            //assert EQ for Some
                                            if kv != None {
                                                //remove Result wrapper
                                                kvc = kv.unwrap();
                                                if kvc.contains("\"txid\":") == true {
                                                    println!("\n");
                                                }
                                                if kvc.len() >= 400 {
                                                    let mut h = String::from(kvc.clone());
                                                    h = h.replace("\"", "");

                                                    loop {
                                                        if h.len() == 0 {
                                                            break;
                                                        }
                                                        let mut x = String::new();
                                                        loop {
                                                            if h.len() == 0 {
                                                                break;
                                                            }
                                                            let y =
                                                                h.remove(0).to_ascii_lowercase();
                                                            if y == '\u{20}' {
                                                                continue;
                                                            }
                                                            x.push(y);
                                                            break;
                                                        }
                                                        loop {
                                                            if h.len() == 0 {
                                                                break;
                                                            }
                                                            let z =
                                                                h.remove(0).to_ascii_lowercase();
                                                            if z == '\u{20}' {
                                                                continue;
                                                            }
                                                            x.push(z);
                                                            break;
                                                        }
                                                        match x.as_str() {
                                                            //decoded output
                                                            "00" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{00}\x1b[0m");
                                                            }
                                                            "01" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{01}\x1b[0m");
                                                            }
                                                            "02" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{02}\x1b[0m");
                                                            }
                                                            "03" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{03}\x1b[0m");
                                                            }
                                                            "04" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{04}\x1b[0m");
                                                            }
                                                            "05" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{05}\x1b[0m");
                                                            }
                                                            "06" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{06}\x1b[0m");
                                                            }
                                                            "07" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{07}\x1b[0m");
                                                            }
                                                            "08" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{08}\x1b[0m");
                                                            }
                                                            "09" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{09}\x1b[0m");
                                                            }
                                                            "0a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0a}\x1b[0m");
                                                            }
                                                            "0b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0b}\x1b[0m");
                                                            }
                                                            "0c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0c}\x1b[0m");
                                                            }
                                                            "0d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0d}\x1b[0m");
                                                            }
                                                            "0e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0e}\x1b[0m");
                                                            }
                                                            "0f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0f}\x1b[0m");
                                                            }
                                                            "10" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{10}\x1b[0m");
                                                            }
                                                            "11" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{11}\x1b[0m");
                                                            }
                                                            "12" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{12}\x1b[0m");
                                                            }
                                                            "13" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{13}\x1b[0m");
                                                            }
                                                            "14" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{14}\x1b[0m");
                                                            }
                                                            "15" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{15}\x1b[0m");
                                                            }
                                                            "16" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{16}\x1b[0m");
                                                            }
                                                            "17" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{17}\x1b[0m");
                                                            }
                                                            "18" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{18}\x1b[0m");
                                                            }
                                                            "19" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{19}\x1b[0m");
                                                            }
                                                            "1a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{1a}\x1b[0m");
                                                            }
                                                            "1b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{1b}\x1b[0m");
                                                            }
                                                            "1c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{1c}\x1b[0m");
                                                            }
                                                            "1d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{1d}\x1b[0m");
                                                            }
                                                            "1e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{1e}\x1b[0m");
                                                            }
                                                            "1f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{1f}\x1b[0m");
                                                            }
                                                            "20" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{20}\x1b[0m");
                                                            }
                                                            "21" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{21}\x1b[0m");
                                                            }
                                                            "22" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{22}\x1b[0m");
                                                            }
                                                            "23" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{23}\x1b[0m");
                                                            }
                                                            "24" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{24}\x1b[0m");
                                                            }
                                                            "25" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{25}\x1b[0m");
                                                            }
                                                            "26" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{26}\x1b[0m");
                                                            }
                                                            "27" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{27}\x1b[0m");
                                                            }
                                                            "28" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{28}\x1b[0m");
                                                            }
                                                            "29" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{29}\x1b[0m");
                                                            }
                                                            "2a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{2a}\x1b[0m");
                                                            }
                                                            "2b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{2b}\x1b[0m");
                                                            }
                                                            "2c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{2c}\x1b[0m");
                                                            }
                                                            "2d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{2d}\x1b[0m");
                                                            }
                                                            "2e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{2e}\x1b[0m");
                                                            }
                                                            "2f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{2f}\x1b[0m");
                                                            }
                                                            "30" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{30}\x1b[0m");
                                                            }
                                                            "31" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{31}\x1b[0m");
                                                            }
                                                            "32" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{32}\x1b[0m");
                                                            }
                                                            "33" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{33}\x1b[0m");
                                                            }
                                                            "34" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{34}\x1b[0m");
                                                            }
                                                            "35" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{35}\x1b[0m");
                                                            }
                                                            "36" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{36}\x1b[0m");
                                                            }
                                                            "37" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{37}\x1b[0m");
                                                            }
                                                            "38" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{38}\x1b[0m");
                                                            }
                                                            "39" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{39}\x1b[0m");
                                                            }
                                                            "3a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{3a}\x1b[0m");
                                                            }
                                                            "3b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{3b}\x1b[0m");
                                                            }
                                                            "3c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{3c}\x1b[0m");
                                                            }
                                                            "3d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{3d}\x1b[0m");
                                                            }
                                                            "3e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{3e}\x1b[0m");
                                                            }
                                                            "3f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{3f}\x1b[0m");
                                                            }
                                                            "40" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{40}\x1b[0m");
                                                            }
                                                            "41" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{41}\x1b[0m");
                                                            }
                                                            "42" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{42}\x1b[0m");
                                                            }
                                                            "43" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{43}\x1b[0m");
                                                            }
                                                            "44" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{44}\x1b[0m");
                                                            }
                                                            "45" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{45}\x1b[0m");
                                                            }
                                                            "46" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{46}\x1b[0m");
                                                            }
                                                            "47" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{47}\x1b[0m");
                                                            }
                                                            "48" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{48}\x1b[0m");
                                                            }
                                                            "49" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{49}\x1b[0m");
                                                            }
                                                            "4a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{4a}\x1b[0m");
                                                            }
                                                            "4b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{4b}\x1b[0m");
                                                            }
                                                            "4c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{4c}\x1b[0m");
                                                            }
                                                            "4d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{4d}\x1b[0m");
                                                            }
                                                            "4e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{4e}\x1b[0m");
                                                            }
                                                            "4f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{4f}\x1b[0m");
                                                            }
                                                            "50" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{50}\x1b[0m");
                                                            }
                                                            "51" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{51}\x1b[0m");
                                                            }
                                                            "52" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{52}\x1b[0m");
                                                            }
                                                            "53" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{53}\x1b[0m");
                                                            }
                                                            "54" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{54}\x1b[0m");
                                                            }
                                                            "55" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{55}\x1b[0m");
                                                            }
                                                            "56" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{56}\x1b[0m");
                                                            }
                                                            "57" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{57}\x1b[0m");
                                                            }
                                                            "58" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{58}\x1b[0m");
                                                            }
                                                            "59" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{59}\x1b[0m");
                                                            }
                                                            "5a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{5a}\x1b[0m");
                                                            }
                                                            "5b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{5b}\x1b[0m");
                                                            }
                                                            "5c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{5c}\x1b[0m");
                                                            }
                                                            "5d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{5d}\x1b[0m");
                                                            }
                                                            "5e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{5e}\x1b[0m");
                                                            }
                                                            "5f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{5f}\x1b[0m");
                                                            }
                                                            "60" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{60}\x1b[0m");
                                                            }
                                                            "61" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{61}\x1b[0m");
                                                            }
                                                            "62" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{62}\x1b[0m");
                                                            }
                                                            "63" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{63}\x1b[0m");
                                                            }
                                                            "64" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{64}\x1b[0m");
                                                            }
                                                            "65" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{65}\x1b[0m");
                                                            }
                                                            "66" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{66}\x1b[0m");
                                                            }
                                                            "67" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{67}\x1b[0m");
                                                            }
                                                            "68" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{68}\x1b[0m");
                                                            }
                                                            "69" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{69}\x1b[0m");
                                                            }
                                                            "6a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{6a}\x1b[0m");
                                                            }
                                                            "6b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{6b}\x1b[0m");
                                                            }
                                                            "6c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{6c}\x1b[0m");
                                                            }
                                                            "6d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{6d}\x1b[0m");
                                                            }
                                                            "6e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{6e}\x1b[0m");
                                                            }
                                                            "6f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{6f}\x1b[0m");
                                                            }
                                                            "70" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{70}\x1b[0m");
                                                            }
                                                            "71" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{71}\x1b[0m");
                                                            }
                                                            "72" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{72}\x1b[0m");
                                                            }
                                                            "73" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{73}\x1b[0m");
                                                            }
                                                            "74" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{74}\x1b[0m");
                                                            }
                                                            "75" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{75}\x1b[0m");
                                                            }
                                                            "76" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{76}\x1b[0m");
                                                            }
                                                            "77" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{77}\x1b[0m");
                                                            }
                                                            "78" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{78}\x1b[0m");
                                                            }
                                                            "79" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{79}\x1b[0m");
                                                            }
                                                            "7a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{7a}\x1b[0m");
                                                            }
                                                            "7b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{7b}\u{7b}\x1b[0m");
                                                            }
                                                            "7c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{7c}\x1b[0m");
                                                            }
                                                            "7d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{7d}\u{7d}\x1b[0m");
                                                            }
                                                            "7e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{7e}\x1b[0m");
                                                            }
                                                            "7f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{7f}\x1b[0m");
                                                            }
                                                            "80" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{80}\x1b[0m");
                                                            }
                                                            "81" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0081}\x1b[0m");
                                                            }
                                                            "82" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0082}\x1b[0m");
                                                            }
                                                            "83" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0083}\x1b[0m");
                                                            }
                                                            "84" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0084}\x1b[0m");
                                                            }
                                                            "85" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0085}\x1b[0m");
                                                            }
                                                            "86" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0086}\x1b[0m");
                                                            }
                                                            "87" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0087}\x1b[0m");
                                                            }
                                                            "88" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0088}\x1b[0m");
                                                            }
                                                            "89" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{0089}\x1b[0m");
                                                            }
                                                            "8a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{008a}\x1b[0m");
                                                            }
                                                            "8b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{008b}\x1b[0m");
                                                            }
                                                            "8c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{8c}\x1b[0m");
                                                            }
                                                            "8d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{8d}\x1b[0m");
                                                            }
                                                            "8e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{8e}\x1b[0m");
                                                            }
                                                            "8f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{8f}\x1b[0m");
                                                            }
                                                            "90" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{90}\x1b[0m");
                                                            }
                                                            "91" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{91}\x1b[0m");
                                                            }
                                                            "92" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{92}\x1b[0m");
                                                            }
                                                            "93" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{93}\x1b[0m");
                                                            }
                                                            "94" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{94}\x1b[0m");
                                                            }
                                                            "95" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{95}\x1b[0m");
                                                            }
                                                            "96" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{96}\x1b[0m");
                                                            }
                                                            "97" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{97}\x1b[0m");
                                                            }
                                                            "98" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{98}\x1b[0m");
                                                            }
                                                            "99" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{99}\x1b[0m");
                                                            }
                                                            "9a" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{9a}\x1b[0m");
                                                            }
                                                            "9b" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{9b}\x1b[0m");
                                                            }
                                                            "9c" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{9c}\x1b[0m");
                                                            }
                                                            "9d" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{9d}\x1b[0m");
                                                            }
                                                            "9e" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{9e}\x1b[0m");
                                                            }
                                                            "9f" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{9f}\x1b[0m");
                                                            }
                                                            "a0" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a0}\x1b[0m");
                                                            }
                                                            "a1" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a1}\x1b[0m");
                                                            }
                                                            "a2" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a2}\x1b[0m");
                                                            }
                                                            "a3" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a3}\x1b[0m");
                                                            }
                                                            "a4" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a4}\x1b[0m");
                                                            }
                                                            "a5" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a5}\x1b[0m");
                                                            }
                                                            "a6" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a6}\x1b[0m");
                                                            }
                                                            "a7" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a7}\x1b[0m");
                                                            }
                                                            "a8" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a8}\x1b[0m");
                                                            }
                                                            "a9" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{a9}\x1b[0m");
                                                            }
                                                            "aa" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{aa}\x1b[0m");
                                                            }
                                                            "ab" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ab}\x1b[0m");
                                                            }
                                                            "ac" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ac}\x1b[0m");
                                                            }
                                                            "ad" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ad}\x1b[0m");
                                                            }
                                                            "ae" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ae}\x1b[0m");
                                                            }
                                                            "af" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{af}\x1b[0m");
                                                            }
                                                            "b0" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b0}\x1b[0m");
                                                            }
                                                            "b1" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b1}\x1b[0m");
                                                            }
                                                            "b2" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b2}\x1b[0m");
                                                            }
                                                            "b3" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b3}\x1b[0m");
                                                            }
                                                            "b4" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b4}\x1b[0m");
                                                            }
                                                            "b5" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b5}\x1b[0m");
                                                            }
                                                            "b6" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b6}\x1b[0m");
                                                            }
                                                            "b7" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b7}\x1b[0m");
                                                            }
                                                            "b8" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b8}\x1b[0m");
                                                            }
                                                            "b9" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{b9}\x1b[0m");
                                                            }
                                                            "ba" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ba}\x1b[0m");
                                                            }
                                                            "bb" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{bb}\x1b[0m");
                                                            }
                                                            "bc" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{bc}\x1b[0m");
                                                            }
                                                            "bd" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{bd}\x1b[0m");
                                                            }
                                                            "be" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{be}\x1b[0m");
                                                            }
                                                            "bf" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{bf}\x1b[0m");
                                                            }
                                                            "c0" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c0}\x1b[0m");
                                                            }
                                                            "c1" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c1}\x1b[0m");
                                                            }
                                                            "c2" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c2}\x1b[0m");
                                                            }
                                                            "c3" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c3}\x1b[0m");
                                                            }
                                                            "c4" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c4}\x1b[0m");
                                                            }
                                                            "c5" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c5}\x1b[0m");
                                                            }
                                                            "c6" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c6}\x1b[0m");
                                                            }
                                                            "c7" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c7}\x1b[0m");
                                                            }
                                                            "c8" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c8}\x1b[0m");
                                                            }
                                                            "c9" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{c9}\x1b[0m");
                                                            }
                                                            "ca" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ca}\x1b[0m");
                                                            }
                                                            "cb" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{cb}\x1b[0m");
                                                            }
                                                            "cc" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{cc}\x1b[0m");
                                                            }
                                                            "cd" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{cd}\x1b[0m");
                                                            }
                                                            "ce" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ce}\x1b[0m");
                                                            }
                                                            "cf" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{cf}\x1b[0m");
                                                            }
                                                            "d0" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d0}\x1b[0m");
                                                            }
                                                            "d1" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d1}\x1b[0m");
                                                            }
                                                            "d2" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d2}\x1b[0m");
                                                            }
                                                            "d3" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d3}\x1b[0m");
                                                            }
                                                            "d4" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d4}\x1b[0m");
                                                            }
                                                            "d5" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d5}\x1b[0m");
                                                            }
                                                            "d6" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d6}\x1b[0m");
                                                            }
                                                            "d7" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d7}\x1b[0m");
                                                            }
                                                            "d8" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d8}\x1b[0m");
                                                            }
                                                            "d9" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{d9}\x1b[0m");
                                                            }
                                                            "da" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{da}\x1b[0m");
                                                            }
                                                            "db" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{db}\x1b[0m");
                                                            }
                                                            "dc" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{dc}\x1b[0m");
                                                            }
                                                            "dd" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{dd}\x1b[0m");
                                                            }
                                                            "de" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{de}\x1b[0m");
                                                            }
                                                            "df" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{df}\x1b[0m");
                                                            }
                                                            "e0" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e0}\x1b[0m");
                                                            }
                                                            "e1" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e1}\x1b[0m");
                                                            }
                                                            "e2" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e2}\x1b[0m");
                                                            }
                                                            "e3" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e3}\x1b[0m");
                                                            }
                                                            "e4" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e4}\x1b[0m");
                                                            }
                                                            "e5" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e5}\x1b[0m");
                                                            }
                                                            "e6" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e6}\x1b[0m");
                                                            }
                                                            "e7" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e7}\x1b[0m");
                                                            }
                                                            "e8" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e8}\x1b[0m");
                                                            }
                                                            "e9" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{e9}\x1b[0m");
                                                            }
                                                            "ea" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ea}\x1b[0m");
                                                            }
                                                            "eb" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{eb}\x1b[0m");
                                                            }
                                                            "ec" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ec}\x1b[0m");
                                                            }
                                                            "ed" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ed}\x1b[0m");
                                                            }
                                                            "ee" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ee}\x1b[0m");
                                                            }
                                                            "ef" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ef}\x1b[0m");
                                                            }
                                                            "f0" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f0}\x1b[0m");
                                                            }
                                                            "f1" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f1}\x1b[0m");
                                                            }
                                                            "f2" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f2}\x1b[0m");
                                                            }
                                                            "f3" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f3}\x1b[0m");
                                                            }
                                                            "f4" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f4}\x1b[0m");
                                                            }
                                                            "f5" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f5}\x1b[0m");
                                                            }
                                                            "f6" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f6}\x1b[0m");
                                                            }
                                                            "f7" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f7}\x1b[0m");
                                                            }
                                                            "f8" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f8}\x1b[0m");
                                                            }
                                                            "f9" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{f9}\x1b[0m");
                                                            }
                                                            "fa" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{fa}\x1b[0m");
                                                            }
                                                            "fb" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{fb}\x1b[0m");
                                                            }
                                                            "fc" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{fc}\x1b[0m");
                                                            }
                                                            "fd" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{fd}\x1b[0m");
                                                            }
                                                            "fe" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{fe}\x1b[0m");
                                                            }
                                                            "ff" => {
                                                                print!("\x1b[48;5;57m\x1b[38;5;188m\u{ff}\x1b[0m");
                                                            }
                                                            _ => (),
                                                        }
                                                        x.clear();
                                                    }
                                                    println!();
                                                    continue;
                                                }

                                                match kvc {
                                                    "[" | "]" | "{" | "}" | "\\" | "}," | "],"
                                                    | "\"addresses\":" | "\"source\":" => {}
                                                    "\"seedfp\":"
                                                    | "\"diversifier_index\":"
                                                    | "\"outindex\":"
                                                    | "\"confirmations\":"
                                                    | "\"blockheight\":"
                                                    | "\"blockindex\":"
                                                    | "\"change\":"
                                                    | "\"pool\":"
                                                    | "\"amountZat\":" => {
                                                        klines.next();
                                                    }

                                                    "\"memo\":" | "\"txid\":" | "\"amount\":" => {
                                                        println!(
                                                            "\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m",
                                                            &kvc
                                                        )
                                                    }

                                                    //match blocktime to a chrono instance
                                                    "\"blocktime\":" => {
                                                        println!(
                                                            "\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m",
                                                            &kvc
                                                        );
                                                        let timeget = klines.next();
                                                        if timeget != None {
                                                            //println!("{:?}", &timeget);
                                                            let mut gottime = timeget
                                                                .clone()
                                                                .expect("")
                                                                .to_string();
                                                            gottime.pop();
                                                            let goime = gottime.parse().expect("");
                                                            //println!("{:?}", &goime);

                                                            #[allow(deprecated)]
                                                            let dt = Utc.timestamp(goime, 0);
                                                            println!("\x1b[48;5;59m\x1b[38;5;187m{}\x1b[0m", &dt);
                                                        }
                                                    }

                                                    _ => println!(
                                                        "\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m",
                                                        &kvc
                                                    ),
                                                }

                                                continue;
                                            }
                                            break;
                                        }
                                    }
                                }

                                let ten_millis = time::Duration::from_millis(50);
                                let now = time::Instant::now();

                                thread::sleep(ten_millis);
                            }

                            //println!("{:?}", &kvc);
                            continue;
                        }
                        break;
                    }
                }
            }

            "A" => {
                {
                    let s5 = String::from(" listaddresses ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    println!("\x1b[48;5;58m\x1b[38;5;188mLegacy Addresses\x1b[0m");

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut addynum = String::new();

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    let mut klines = k.split_whitespace();
                    let mut counter = 1;
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();

                            if kvc.len() >= 24 {
                                if kvc.starts_with("\"t1") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                    continue;
                                }
                                if kvc.starts_with("\"zs") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                    continue;
                                }
                                if kvc.starts_with("\"u1") {
                                    println!();
                                    println!("\x1b[48;5;58m\x1b[38;5;187mUnified Address\x1b[0m");

                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    counter = counter + 1;
                                    addynum.push(' ');

                                    {
                                        {
                                            let s5 = String::from(" z_listunifiedreceivers ");

                                            let mut zadd = String::from(kvc.clone());

                                            zadd.pop();
                                            zadd.push(' ');

                                            let zaddy = zadd.replace('"', "");
                                            //println!("zaddy   {}", &zaddy);
                                            //println!("zaddybg {:?}", &zaddy);

                                            let mut resend0 = resend.clone();
                                            let resend0000 = s5.as_str();

                                            resend0.push_str(resend00);
                                            resend0.push_str(resend000);
                                            resend0.push_str(resend0000);
                                            resend0.push_str(zaddy.as_str());
                                            resend0.push_str(resend00000x);

                                            //println!("{}\n", &resend0);

                                            let mut var_1 = File::create(&rp).expect("no");
                                            var_1.write_all(&resend0.as_bytes()).expect("no");
                                        }

                                        let mut inotify = Inotify::init()
                                            .expect("Error while initializing inotify instance");
                                        inotify
                                            .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                            .expect("Failed to add file watch");

                                        let mut buffer = [0; 512];
                                        let events = inotify
                                            .read_events_blocking(&mut buffer)
                                            .expect("Error while reading events");
                                        #[allow(unused_variables)]
                                        for event in events {
                                            let k = fs::read_to_string(&rp2).unwrap();
                                            println!("\x1b[48;5;58m\x1b[38;5;187mReceivers\x1b[0m");
                                            let mut klines = k.split_whitespace();
                                            loop {
                                                //get first line, if any
                                                let kv = klines.next().clone();
                                                //assert EQ for Some
                                                if kv != None {
                                                    //remove Result wrapper
                                                    kvc = kv.unwrap();

                                                    if kvc.len() >= 24 {
                                                        println!("\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m", &counter.to_string(), &kvc);

                                                        let takeme = kvc.clone();
                                                        let takenum = counter.to_string().clone();
                                                        addynum.push_str(&takenum);
                                                        addynum.push(' ');
                                                        addynum.push_str(takeme);
                                                        addynum.push(' ');
                                                        counter = counter + 1;
                                                        //println!("taken!");
                                                    }
                                                    continue;
                                                }
                                                break;
                                            }

                                            let ten_millis = time::Duration::from_millis(50);
                                            let now = time::Instant::now();

                                            thread::sleep(ten_millis);
                                        }
                                    }

                                    continue;
                                }
                            }

                            continue;
                        }
                        break;
                    }
                }
            }

            "B" => {
                {
                    println!();
                    println!("\x1b[48;5;57m\x1b[38;5;188mPlease Wait........\x1b[0m");
                    println!();

                    let s5 = String::from(" listaddresses ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    println!();

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut addynum = String::new();

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    let mut klines = k.split_whitespace();
                    let mut counter = 1;
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();

                            if kvc.len() >= 24 {
                                if kvc.starts_with("\"t1") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                }
                                if kvc.starts_with("\"zs") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                }
                                if kvc.starts_with("\"u1") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    counter = counter + 1;
                                    addynum.push(' ');
                                }

                                {
                                    {
                                        let s5 = String::from(" z_getbalance ");

                                        let mut zaddd = String::from(kvc.clone());

                                        zaddd.pop();
                                        zaddd.push(' ');

                                        let zadddy = zaddd.replace('"', "");
                                        //println!("zaddy   {}", &zaddy);
                                        //println!("zaddybg {:?}", &zaddy);

                                        let mut resend0 = resend.clone();
                                        let resend0000 = s5.as_str();

                                        resend0.push_str(resend00);
                                        resend0.push_str(resend000);
                                        resend0.push_str(resend0000);
                                        resend0.push_str(zadddy.as_str());
                                        resend0.push_str(resend00000x);

                                        //println!("{}\n", &resend0);

                                        let mut var_1 = File::create(&rp).expect("no");
                                        var_1.write_all(&resend0.as_bytes()).expect("no");
                                    }

                                    let mut inotify = Inotify::init()
                                        .expect("Error while initializing inotify instance");
                                    inotify
                                        .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                        .expect("Failed to add file watch");

                                    let mut buffer = [0; 512];
                                    let events = inotify
                                        .read_events_blocking(&mut buffer)
                                        .expect("Error while reading events");
                                    #[allow(unused_variables)]
                                    for event in events {
                                        let mut k = fs::read_to_string(&rp2).unwrap();

                                        k.pop();
                                        println!("\x1b[48;5;23m\x1b[38;5;187m\"Balance\":\x1b[0m");

                                        println!("\x1b[48;5;23m\x1b[38;5;187m\"{}\"\x1b[0m\n", &k);
                                        let takesum = String::from(&k).clone();
                                        addynum.push_str(&takesum);
                                        addynum.push(' ');
                                    }

                                    //let ten_millis = time::Duration::from_millis(50);
                                    //let now = time::Instant::now();

                                    //thread::sleep(ten_millis);
                                }

                                //println!("\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m", &kvc);

                                let ten_millis = time::Duration::from_millis(50);
                                let now = time::Instant::now();

                                thread::sleep(ten_millis);

                                continue;
                            }

                            //////////////////////////////////////////////////////////////////////////////
                            match kvc {
                                "[" | "]" | "{" | "}" | "\\" | "}," | "]," | "\"addresses\":"
                                | "\"source\":" => {}
                                "\"seedfp\":" | "\"diversifier_index\":" => {
                                    klines.next();
                                }

                                "\"address\":" => {
                                    println!("\x1b[48;5;59m\x1b[38;5;187m{}\x1b[0m", &kvc)
                                }

                                "\"account\":" | "\"receiver_types\":" => {
                                    println!("\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m", &kvc)
                                }

                                _ => println!("\x1b[48;5;53m\x1b[38;5;187m{}\x1b[0m", &kvc),
                            }

                            continue;
                        }
                        break;
                    }

                    {
                        {
                            println!();
                            println!("\x1b[48;5;23m\x1b[38;5;187mTotal Balances:\x1b[0m");

                            let s5 = String::from(" z_gettotalbalance ");
                            let mut resend0 = resend.clone();
                            let resend0000 = s5.as_str();

                            resend0.push_str(resend00);
                            resend0.push_str(resend000);
                            resend0.push_str(resend0000);
                            resend0.push_str(resend00000y);

                            //println!("{}\n", &resend0);

                            let mut var_1 = File::create(&tp).expect("no");
                            var_1.write_all(&resend0.as_bytes()).expect("no");
                        }

                        let mut inotify =
                            Inotify::init().expect("Error while initializing inotify instance");
                        inotify
                            .add_watch(&tp2, WatchMask::CLOSE_WRITE)
                            .expect("Failed to add file watch");

                        let mut buffer = [0; 512];
                        let events = inotify
                            .read_events_blocking(&mut buffer)
                            .expect("Error while reading events");
                        #[allow(unused_variables)]
                        for event in events {
                            let k = fs::read_to_string(&tp2).unwrap();
                            let mut klines = k.split_whitespace();
                            loop {
                                //get first line, if any
                                let kv = klines.next().clone();
                                //assert EQ for Some
                                if kv != None {
                                    //remove Result wrapper
                                    kvc = kv.unwrap();
                                    if kvc.starts_with("\"t") {
                                        println!("\x1b[48;5;53m\x1b[38;5;187m{}\x1b[0m", &kvc);
                                        continue;
                                    }
                                    if kvc.starts_with("\"p") {
                                        println!("\x1b[48;5;53m\x1b[38;5;187m{}\x1b[0m", &kvc);
                                        continue;
                                    }
                                    match kvc {
                                        "[" | "]" | "{" | "}" | "\\" | "}," | "],"
                                        | "\"addresses\":" | "\"source\":" => {}
                                        "\"seedfp\":" | "\"diversifier_index\":" => {
                                            klines.next();
                                        }
                                        _ => println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &kvc),
                                    }
                                    if kvc.len() >= 24 {
                                        println!();
                                    }
                                    continue;
                                }

                                //println!("{}", &addynum);

                                break;
                            }
                        }
                    }

                    let zaddynum = addynum.replace(',', "");
                    //println!("{}", &zaddynum);

                    let mut input = String::new();
                    println!("\x1b[48;5;58m\x1b[38;5;188mEnter Address Number #: to Select or Enter Any Other Input to Return\x1b[0m");
                    io::stdin().read_line(&mut input).unwrap();
                    input.pop();

                    let mut klines = zaddynum.split_whitespace();
                    'outer: loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some

                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();
                            //match here
                            if input == kvc {
                                let seladdy = klines.next().clone().unwrap();
                                let selbal = klines.next().clone().unwrap();

                                //println!("seladdy {:?}\nselbal {:?}", &seladdy, &selbal);
                                println!();
                                println!("\x1b[48;5;52m\x1b[38;5;187m\"Address\": \x1b[0m");
                                if seladdy.starts_with("\"t1") {
                                    println!("\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m", &seladdy);
                                    println!("\x1b[48;5;23m\x1b[38;5;187m\"Balance\": \x1b[0m");
                                    println!("\x1b[48;5;23m\x1b[38;5;187m{} \x1b[0m", &selbal);
                                    println!();
                                }
                                if seladdy.starts_with("\"zs") {
                                    println!("\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m", &seladdy);
                                    println!("\x1b[48;5;23m\x1b[38;5;187m\"Balance\": \x1b[0m");
                                    println!("\x1b[48;5;23m\x1b[38;5;187m{} \x1b[0m", &selbal);
                                    println!();
                                }
                                if seladdy.starts_with("\"u1") {
                                    println!("\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m", &seladdy);
                                    println!("\x1b[48;5;23m\x1b[38;5;187m\"Balance\": \x1b[0m");
                                    println!("\x1b[48;5;23m\x1b[38;5;187m{} \x1b[0m", &selbal);
                                    println!();

                                    {
                                        {
                                            let s5 = String::from(" z_listunifiedreceivers ");

                                            let mut zadd = String::from(seladdy.clone());

                                            zadd.pop();
                                            zadd.push(' ');

                                            let zaddy = zadd.replace('"', "");
                                            //println!("zaddy   {}", &zaddy);
                                            //println!("zaddybg {:?}", &zaddy);

                                            let mut resend0 = resend.clone();
                                            let resend0000 = s5.as_str();

                                            resend0.push_str(resend00);
                                            resend0.push_str(resend000);
                                            resend0.push_str(resend0000);
                                            resend0.push_str(zaddy.as_str());
                                            resend0.push_str(resend00000x);

                                            //println!("{}\n", &resend0);

                                            let mut var_1 = File::create(&rp).expect("no");
                                            var_1.write_all(&resend0.as_bytes()).expect("no");
                                        }

                                        let mut inotify = Inotify::init()
                                            .expect("Error while initializing inotify instance");
                                        inotify
                                            .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                            .expect("Failed to add file watch");

                                        let mut buffer = [0; 512];
                                        let events = inotify
                                            .read_events_blocking(&mut buffer)
                                            .expect("Error while reading events");
                                        #[allow(unused_variables)]
                                        for event in events {
                                            let k = fs::read_to_string(&rp2).unwrap();
                                            println!("\x1b[48;5;58m\x1b[38;5;187mReceivers\x1b[0m");
                                            let mut klines = k.split_whitespace();
                                            loop {
                                                //get first line, if any
                                                let kv = klines.next().clone();
                                                //assert EQ for Some
                                                if kv != None {
                                                    //remove Result wrapper
                                                    kvc = kv.unwrap();

                                                    if kvc.len() >= 24 {
                                                        println!(
                                                            "\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m",
                                                            &kvc
                                                        );
                                                    }
                                                    continue;
                                                }
                                                break;
                                            }
                                            let ten_millis = time::Duration::from_millis(50);
                                            let now = time::Instant::now();

                                            thread::sleep(ten_millis);
                                        }
                                    }
                                }

                                let mut enof = false;
                                let numfromstr: f32 = selbal.parse().unwrap();
                                if numfromstr >= 0.00001 {
                                    enof = true;
                                    println!();

                                    println!("\x1b[48;5;54m\x1b[38;5;188m**Yes Enough Funds To Send**\x1b[0m");
                                    println!();
                                }

                                if enof == false {
                                    println!();

                                    println!("\x1b[48;5;52m\x1b[38;5;188m**Not Enough Funds To Send**\x1b[0m");
                                    println!();

                                    break;
                                }

                                let mut input = String::new();

                                println!("\x1b[48;5;58m\x1b[38;5;188mEnter  S  to Send Funds or Enter Any Other Input to Return\x1b[0m");
                                println!("\x1b[48;5;58m\x1b[38;5;188mEnter Any Other Input to Return\x1b[0m");

                                io::stdin().read_line(&mut input).unwrap();
                                input.pop();
                                if input.as_str() != "S" {
                                    break;
                                }

                                //println!(" {:?}\n {:?}", &seladdy, &selbal);

                                //////////////////////////////////
                                //////////////////////////////////
                                //Start taking the Receiver, amount, priv
                                //policy if applicable, memo plus encoded,
                                //show all to confirm before send, type "S"
                                //final are you sure? type "yes"
                                let mut input = String::new();
                                println!("\x1b[48;5;54m\x1b[38;5;188mSend To: Paste In The Receiver Address\x1b[0m");
                                io::stdin().read_line(&mut input).unwrap();
                                input.pop();

                                 let goodaddy1 = String::from(&input).clone();
				let mut goodaddy = goodaddy1.replace('"', "");

                                {
                                    {
                                        let s5 = String::from(" z_validateaddress ");

                                        let mut zadd = String::from(input).clone();

                                        zadd.push(' ');

                                        let zaddy = zadd.replace('"', "");
                                        //println!("zaddy   {}", &zaddy);
                                        //println!("zaddybg {:?}", &zaddy);

                                        let mut resend0 = resend.clone();
                                        let resend0000 = s5.as_str();

                                        resend0.push_str(resend00);
                                        resend0.push_str(resend000);
                                        resend0.push_str(resend0000);
                                        resend0.push_str(zaddy.as_str());
                                        resend0.push_str(resend00000x);

                                        //println!("{}\n", &resend0);

                                        let mut var_1 = File::create(&rp).expect("no");
                                        var_1.write_all(&resend0.as_bytes()).expect("no");
                                    }

                                    let mut inotify = Inotify::init()
                                        .expect("Error while initializing inotify instance");
                                    inotify
                                        .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                        .expect("Failed to add file watch");

                                    let mut buffer = [0; 512];
                                    let events = inotify
                                        .read_events_blocking(&mut buffer)
                                        .expect("Error while reading events");
                                    #[allow(unused_variables)]
                                    for event in events {
                                        let k = fs::read_to_string(&rp2).unwrap();
                                        let mut klines = k.split_whitespace();
                                        loop {
                                            //get first line, if any
                                            let kv = klines.next().clone();
                                            //assert EQ for Some
                                            if kv != None {
                                                //remove Result wrapper
                                                kvc = kv.unwrap();

                                                match kvc {
                                                    "[" | "]" | "{" | "}" | "\\" | "}," | "],"
                                                    | "\"addresses\":" | "\"source\":" => {}
                                                    "\"seedfp\":" | "\"diversifier_index\":" => {
                                                        klines.next();
                                                    }
                                                    _ => (),
                                                }
                                                if kvc.starts_with("\"isvalid\"") {
                                                    let bewl = klines.next().clone();
                                                    bewl.unwrap();
                                                    //println!("bewl   {:?}", &bewl);

                                                    if bewl.expect("bewlfail").starts_with("false")
                                                    {
                                                        println!("\x1b[48;5;52m\x1b[38;5;187mThe Address is Invalid, Please Try Again\x1b[0m");
                                                        {
                                                            break 'outer;
                                                        }
                                                    }
                                                }

                                                continue;
                                            }
                                            break;
                                        }
                                        //println!("\x1b[48;5;23m\x1b[38;5;187m{:?}\x1b[0m", &k);
                                    }
                                }
                                ////////////////////////////////////////
                                /////////////////////////////////////////
                                //////////////////////////////////////////
                                //here get more input for building the tx command
                                //amount, check against balance
                                //memo, encoded
                                //that that should be all, leave priv policy as
                                //default?
                                //should resemble'
                                //zcash-cli z_sendmany “t1M72Sfpbz1BPpXFHz9m3CdqATR44Jvaydd” ‘[{“address”: “ztfaW34Gj9FrnGUEf833ywDVL62NWXBM81u6EQnM6VR45eYnXhwztecW1SjxA7JrmAXKJhxhj3vDNEpVCQoSvVoSpmbhtjf”, “amount”: 5.0}]’ 10 0.00001 “AllowRevealedSenders”

                                println!("\x1b[48;5;58m\x1b[38;5;188mAddress is Valid\x1b[0m");
                                println!();

                                let mut input = String::new();
                                println!(
                                    "\x1b[48;5;54m\x1b[38;5;188mEnter in the Amount to Send\x1b[0m"
                                );
                                 println!(
                                    "\x1b[48;5;52m\x1b[38;5;188m ! This Amount is Not Verified ! \x1b[0m"
                                );
                                io::stdin().read_line(&mut input).unwrap();
                                input.pop();

                                let the_goodsum = String::from(&input).clone();
                                //let sumfromstr : f32 = the_goodsum.parse().unwrap();

                                //get a memo!!
                                //encode it!
                                println!();

                                let mut input = String::new();
                                println!("\x1b[48;5;54m\x1b[38;5;188mEnter in the Memo, Max length 512\x1b[0m");

                                io::stdin().read_line(&mut input).unwrap();
                                input.pop();

                                #[allow(unused_variables)]
                                let mut postmemo: String;

                                {
                                    let mut h = String::from(input.clone());
                                    h = h.replace("\"", "");

                                    //
                                    let mut s = String::new();
                                    for &c in h.as_bytes() {
                                        write!(&mut s, "{:02X}", c).expect("err");
                                    }

                                    postmemo = s.replace(" ", "").to_ascii_lowercase().clone();

                                    println!();
                                    println!("\x1b[48;5;58m\x1b[38;5;188m Memo: \x1b[0m");

                                    println!("\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m", &h);
                                }

                                //println!("{}", &postmemo);
                                println!();

                                //Add zeroes to tail of post memo, 1024

                                loop {
									
									if postmemo.len() >= 1025 {
										println!("\x1b[48;5;52m\x1b[38;5;188mMemo is Too long\x1b[0m");
										break 'outer;

									}
                                    if postmemo.len() != 1024 {
                                        postmemo.push('0');
                                        continue;
                                    }
                                    break;
                                }
                                //println!("\ngoodaddy {:?}\nselbal {:?}\nseladdy {:?}\nthe_goodsum {:?}", &goodaddy, &selbal, &seladdy, &the_goodsum);

                                println!("\x1b[48;5;52m\x1b[38;5;188m From: {}\x1b[0m", &seladdy);
                                println!(
                                    "\x1b[48;5;53m\x1b[38;5;188m Amount : {}\x1b[0m",
                                    &the_goodsum
                                );
                                println!("\x1b[48;5;52m\x1b[38;5;188m To: {}\x1b[0m", &goodaddy);
                                println!("\x1b[48;5;53m\x1b[38;5;188m Fee: 0.00001\x1b[0m");

                                println!();

                                println!("\x1b[48;5;54m\x1b[38;5;188m Please Review Tx Details!  \x1b[0m");

                                println!();

                                println!("\x1b[48;5;52m\x1b[38;5;188m ! Enter  Y  to Send \x1b[0m");
                                println!("\x1b[48;5;54m\x1b[38;5;188m Enter Any Other Input to Return \x1b[0m");

                                let mut zadd = String::new();

                                io::stdin().read_line(&mut zadd).unwrap();
                                zadd.pop();
                                let yes = "Y";

                                let mut zaddy = zadd.replace('"', "");
                                //println!("zaddy   {}", &zaddy);
                                //println!("zaddybg {:?}", &zaddy);

                                if zaddy.contains(&yes) != true {
                                    goodaddy.push_str("make_fail");
                                    break;
                                    //println!("\x1b[48;5;52m\x1b[38;5;187mInvalid Input, Please Try Again\x1b[0m");
                                }
                                if zaddy.contains(&yes) == true {
                                    zaddy.clear();
                                }

                                {
                                    {
                                        let s5 = String::from(" z_sendmany ");

                                        let mut resend0 = resend.clone();
                                        let resend0000 = s5.as_str();

                                        resend0.push_str(resend00);
                                        resend0.push_str(resend000);
                                        resend0.push_str(resend0000);
                                        resend0.push_str(seladdy);
                                        resend0.push(' ');
                                        resend0.push('\'');
                                        resend0.push('[');
                                        resend0.push('{');

                                        resend0.push_str("\"address\":");
                                        resend0.push('"');
                                        resend0.push_str(&goodaddy);
                                        resend0.push('"');

                                        resend0.push(',');

                                        resend0.push_str("\"amount\":");
                                        resend0.push_str(&the_goodsum);

                                        resend0.push(',');

                                        resend0.push_str("\"memo\":");
                                        resend0.push('"');
                                        resend0.push_str(&postmemo);
                                        resend0.push('"');

                                        resend0.push('}');
                                        resend0.push(']');
                                        resend0.push('\'');
                                        resend0.push(' ');
					//NoPrivacy allows wallet to execute whatever actions necessary to complete.
					//Useful because you can't get the UA receiver balances natively.
                                        resend0.push_str("10 0.00001 \"NoPrivacy\" ");

                                        resend0.push_str(resend00000);

                                        //println!("{}\n", &resend0);

                                        let mut var_1 = File::create(&sp).expect("no");
                                        var_1.write_all(&resend0.as_bytes()).expect("no");
                                    }

                                    let mut inotify = Inotify::init()
                                        .expect("Error while initializing inotify instance");
                                    inotify
                                        .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                                        .expect("Failed to add file watch");

                                    let mut buffer = [0; 512];
                                    let events = inotify
                                        .read_events_blocking(&mut buffer)
                                        .expect("Error while reading events");
                                    #[allow(unused_variables)]
                                    for event in events {
                                        let mut k = fs::read_to_string(&sp2).unwrap();

                                        k.pop();

                                        println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &k);
                                    }
                                }
                            }
                            continue;
                        }
                        break;
                    }
                }
            }

            "F" => {
                {
                    let s5 = String::from(" z_getaddressforaccount ");

                    println!("\x1b[48;5;58m\x1b[38;5;188mEnter the account number #\x1b[0m");
                    let mut dothexist = String::new();

                    {
                        {
                            let s5 = String::from(" z_listaccounts ");

                            let mut resend0 = resend.clone();
                            let resend0000 = s5.as_str();

                            resend0.push_str(resend00);
                            resend0.push_str(resend000);
                            resend0.push_str(resend0000);
                            resend0.push_str(resend00000);

                            //println!("{}\n", &resend0);

                            let mut var_1 = File::create(&sp).expect("no");
                            var_1.write_all(&resend0.as_bytes()).expect("no");
                        }

                        println!("\x1b[48;5;54m\x1b[38;5;188mExisting Accounts - \x1b[0m");
                        let mut inotify =
                            Inotify::init().expect("Error while initializing inotify instance");
                        inotify
                            .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                            .expect("Failed to add file watch");

                        let mut buffer = [0; 512];
                        let events = inotify
                            .read_events_blocking(&mut buffer)
                            .expect("Error while reading events");
                        #[allow(unused_variables)]
                        for event in events {
                            let k = fs::read_to_string(&sp2).unwrap();
                            let mut klines = k.split_whitespace();
                            loop {
                                //get first line, if any
                                let kv = klines.next().clone();
                                //assert EQ for Some
                                if kv != None {
                                    //remove Result wrapper
                                    kvc = kv.unwrap();

                                    match kvc {
                                        "\"account\":" => {
                                            let doex = &klines.next().unwrap();
                                            println!("\x1b[48;5;54m\x1b[38;5;188m{}\x1b[0m", &doex);
                                            dothexist.push_str(doex);
                                        }

                                        _ => (),
                                    }

                                    continue;
                                }

                                //println!("dothexist {}", &dothexist);

                                break;
                            }
                        }
                    }

                    let mut zadd = String::new();

                    io::stdin().read_line(&mut zadd).unwrap();
                    zadd.pop();

                    let mut zaddy = zadd.replace('"', "");
                    //println!("zaddy   {}", &zaddy);
                    //println!("zaddybg {:?}", &zaddy);

                    let mut laddy = String::new();
                    let mut faddy = String::new();

                    if dothexist.contains(&zaddy) != true {
                        zaddy.push_str("make_fail");

                        println!(
                            "\x1b[48;5;52m\x1b[38;5;187mInvalid Input,  Please Try Again\x1b[0m"
                        );
                    }
                    if zaddy.len() <= 0 {
                        zaddy.push_str("make_fail");

                        println!(
                            "\x1b[48;5;52m\x1b[38;5;187mInvalid Input,  Please Try Again\x1b[0m"
                        );
                    }

                    if dothexist.contains(&zaddy) == true {
                        //println!("OK!");

                        println!("\x1b[48;5;58m\x1b[38;5;187mEnter the Unified Receivers\x1b[0m");
                        println!("\x1b[48;5;54m\x1b[38;5;187m1:  p2pkh , sapling, orchard\x1b[0m");
                        println!("\x1b[48;5;54m\x1b[38;5;187m2:  sapling, orchard\x1b[0m");
                        println!("\x1b[48;5;54m\x1b[38;5;187m3:  p2pkh, orchard\x1b[0m");
                        println!("\x1b[48;5;54m\x1b[38;5;187m4: p2pkh,  sapling\x1b[0m");
                        println!("\x1b[48;5;54m\x1b[38;5;187m5: orchard\x1b[0m");
                        println!("\x1b[48;5;54m\x1b[38;5;187m6: sapling\x1b[0m");

                        io::stdin().read_line(&mut laddy).unwrap();
                        laddy.pop();

                        let laddy = laddy.replace('"', "");
                        //println!("laddy   {}", &laddy);
                        //println!("laddybg {:?}", &laddy);

                        //match to ladd 1-6 and code on while true
                        //convert ladd to the appropriate str like  '["p2pkh","sapling","orchard"]'
                        //would be new string, push '-[-"-str-"-,-"-str-"-,-"-str-"-]-'
                        //remember spaces before after

                        match laddy.as_str() {
                            "1" => {
                                faddy.push(' ');
                                faddy.push('\'');
                                faddy.push('[');

                                faddy.push_str("\"p2pkh\"");

                                faddy.push(',');

                                faddy.push_str("\"sapling\"");

                                faddy.push(',');

                                faddy.push_str("\"orchard\"");

                                faddy.push(']');
                                faddy.push('\'');
                                faddy.push(' ');
                            }

                            "2" => {
                                faddy.push(' ');
                                faddy.push('\'');
                                faddy.push('[');

                                faddy.push_str("\"sapling\"");

                                faddy.push(',');

                                faddy.push_str("\"orchard\"");

                                faddy.push(']');
                                faddy.push('\'');
                                faddy.push(' ');
                            }

                            "3" => {
                                faddy.push(' ');
                                faddy.push('\'');
                                faddy.push('[');

                                faddy.push_str("\"p2pkh\"");

                                faddy.push(',');

                                faddy.push_str("\"orchard\"");

                                faddy.push(']');
                                faddy.push('\'');
                                faddy.push(' ');
                            }

                            "4" => {
                                faddy.push(' ');
                                faddy.push('\'');
                                faddy.push('[');

                                faddy.push_str("\"p2pkh\"");

                                faddy.push(',');

                                faddy.push_str("\"sapling\"");

                                faddy.push(']');
                                faddy.push('\'');
                                faddy.push(' ');
                            }

                            "5" => {
                                faddy.push(' ');
                                faddy.push('\'');
                                faddy.push('[');

                                faddy.push_str("\"orchard\"");

                                faddy.push(']');
                                faddy.push('\'');
                                faddy.push(' ');
                            }

                            "6" => {
                                faddy.push(' ');
                                faddy.push('\'');
                                faddy.push('[');

                                faddy.push_str("\"sapling\"");

                                faddy.push(']');
                                faddy.push('\'');
                                faddy.push(' ');
                            }

                            //make it fail the call
                            _ => {
                                faddy.push_str("make_fail");
                                println!("\x1b[48;5;52m\x1b[38;5;187mInvalid Input, Please Try Again\x1b[0m");
                            }
                        }
                    }

                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(zaddy.as_str());
                    resend0.push_str(faddy.as_str());

                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let mut k = fs::read_to_string(&sp2).unwrap();

                    k.pop();
                    if k.contains("error") != true {
                        //Add match to "account" and "address"?
                        //idk...

                        println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &k);
                    }
                }
            }

            "N" => {
                {
                    //println!("N - z_getnewaccount ok!");

                    let s5 = String::from(" z_getnewaccount ");
                    let mut dothexist = String::new();

                    println!("\x1b[48;5;54m\x1b[38;5;188mExisting Accounts #\x1b[0m");

                    {
                        {
                            let s5 = String::from(" z_listaccounts ");

                            let mut resend0 = resend.clone();
                            let resend0000 = s5.as_str();

                            resend0.push_str(resend00);
                            resend0.push_str(resend000);
                            resend0.push_str(resend0000);
                            resend0.push_str(resend00000);

                            //println!("{}\n", &resend0);

                            let mut var_1 = File::create(&sp).expect("no");
                            var_1.write_all(&resend0.as_bytes()).expect("no");
                        }

                        let mut inotify =
                            Inotify::init().expect("Error while initializing inotify instance");
                        inotify
                            .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                            .expect("Failed to add file watch");

                        let mut buffer = [0; 512];
                        let events = inotify
                            .read_events_blocking(&mut buffer)
                            .expect("Error while reading events");
                        #[allow(unused_variables)]
                        for event in events {
                            let k = fs::read_to_string(&sp2).unwrap();
                            let mut klines = k.split_whitespace();
                            loop {
                                //get first line, if any
                                let kv = klines.next().clone();
                                //assert EQ for Some
                                if kv != None {
                                    //remove Result wrapper
                                    kvc = kv.unwrap();

                                    match kvc {
                                        "\"account\":" => {
                                            let doex = &klines.next().unwrap();
                                            println!("\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m", &doex);
                                            dothexist.push_str(doex);
                                        }

                                        _ => (),
                                    }

                                    continue;
                                }

                                //println!("\x1b[48;5;58m\x1b[38;5;188m{}\x1b[0m", &dothexist);

                                break;
                            }
                        }
                    }
                    println!("\x1b[48;5;54m\x1b[38;5;188mEnter  Y  to Create a New Account\x1b[0m");
                    println!("\x1b[48;5;54m\x1b[38;5;188mEnter Any Other Input to Return\x1b[0m");

                    let mut zadd = String::new();

                    io::stdin().read_line(&mut zadd).unwrap();
                    zadd.pop();
                    let yes = "Y";

                    let mut zaddy = zadd.replace('"', "");
                    //println!("zaddy   {}", &zaddy);
                    //println!("zaddybg {:?}", &zaddy);

                    if zaddy.contains(&yes) != true {
                        zaddy.push_str("make_fail");
                    }
                    if zaddy.contains(&yes) == true {
                        zaddy.clear();
                    }

                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(zaddy.as_str());
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let mut k = fs::read_to_string(&sp2).unwrap();

                    k.pop();
                    if k.contains("error") != true {
                        println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &k);

                        //Add match to "account" #?
                        //idk
                    }
                }
            }

            "C" => {
                {
                    //println!("C - z_getbalance ok!");

                    let s5 = String::from(" z_getbalance ");

                    println!("\x1b[48;5;58m\x1b[38;5;188mPaste in your address below\x1b[0m");

                    let mut zadd = String::new();

                    io::stdin().read_line(&mut zadd).unwrap();
                    zadd.pop();
                    zadd.push(' ');

                    let zaddy = zadd.replace('"', "");
                    //println!("zaddy   {}", &zaddy);
                    //println!("zaddybg {:?}", &zaddy);

                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(zaddy.as_str());
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let mut k = fs::read_to_string(&sp2).unwrap();

                    k.pop();
                    if k.contains("error") != true {
                        println!("\x1b[48;5;23m\x1b[38;5;187mBalance:\x1b[0m");
                        println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &k);
                    }
                    if k.contains("error") == true {
                        println!(
                            "\x1b[48;5;52m\x1b[38;5;187mInvalid Input, Please Try Again\x1b[0m"
                        );
                    }
                }
            }

		  "S" => {
                {
                    let s5 = String::from(" listaddresses ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    println!("\x1b[48;5;58m\x1b[38;5;188mLegacy Addresses\x1b[0m");

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut addynum = String::new();

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    let mut klines = k.split_whitespace();
                    let mut counter = 1;
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();
				if kvc.starts_with("\"account") {
				println!();
				println!("\x1b[48;5;57m\x1b[38;5;187mAccount:  {}\x1b[0m", &klines.next().unwrap());
                                    
								}
                            
                            if kvc.len() >= 24 {
                                if kvc.starts_with("\"t1") {
                                    println!(
                                        "\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m",
                                        &counter.to_string(),
                                        &kvc
                                    );

                                    let takeme = kvc.clone();
                                    let takenum = counter.to_string().clone();
                                    addynum.push_str(&takenum);
                                    addynum.push(' ');
                                    addynum.push_str(takeme);
                                    addynum.push(' ');
                                    counter = counter + 1;
                                    continue;
                                }
                               
                                if kvc.starts_with("\"u1") {
                                    
                                   

                                    {
                                        {
                                            let s5 = String::from(" z_listunifiedreceivers ");

                                            let mut zadd = String::from(kvc.clone());

                                            zadd.pop();
                                            zadd.push(' ');

                                            let zaddy = zadd.replace('"', "");
                                            //println!("zaddy   {}", &zaddy);
                                            //println!("zaddybg {:?}", &zaddy);

                                            let mut resend0 = resend.clone();
                                            let resend0000 = s5.as_str();

                                            resend0.push_str(resend00);
                                            resend0.push_str(resend000);
                                            resend0.push_str(resend0000);
                                            resend0.push_str(zaddy.as_str());
                                            resend0.push_str(resend00000x);

                                            //println!("{}\n", &resend0);

                                            let mut var_1 = File::create(&rp).expect("no");
                                            var_1.write_all(&resend0.as_bytes()).expect("no");
                                        }

                                        let mut inotify = Inotify::init()
                                            .expect("Error while initializing inotify instance");
                                        inotify
                                            .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                            .expect("Failed to add file watch");

                                        let mut buffer = [0; 512];
                                        let events = inotify
                                            .read_events_blocking(&mut buffer)
                                            .expect("Error while reading events");
                                        #[allow(unused_variables)]
                                        for event in events {
                                            let k = fs::read_to_string(&rp2).unwrap();
                                            
                                            let mut klines = k.split_whitespace();
                                            loop {
                                                //get first line, if any
                                                let kv = klines.next().clone();
                                                //assert EQ for Some
                                                if kv != None {
                                                    //remove Result wrapper
                                                    kvc = kv.unwrap();

                                                    if kvc.starts_with("\"t1") {
                                                    println!("\x1b[48;5;58m\x1b[38;5;187mReceivers\x1b[0m");
                                                        println!("\x1b[48;5;52m\x1b[38;5;187m{}:  {}\x1b[0m", &counter.to_string(), &kvc);

                                                        let takeme = kvc.clone();
                                                        let takenum = counter.to_string().clone();
                                                        addynum.push_str(&takenum);
                                                        addynum.push(' ');
                                                        addynum.push_str(takeme);
                                                        addynum.push(' ');
                                                        counter = counter + 1;
                                                        //println!("taken!");
                                                    }
                                                    continue;
                                                }
                                                break;
                                            }

                                            let ten_millis = time::Duration::from_millis(50);
                                            let now = time::Instant::now();

                                            thread::sleep(ten_millis);
                                        }
                                    }

                                    continue;
                                }
                            }

                            continue;
                        }
                        break;
                    }
                }
                
                
                let zaddynum = addynum.replace(',', "");
                    //println!("{}", &zaddynum);
                    println!("\x1b[48;5;58m\x1b[38;5;188mEnter Address Number to Select\x1b[0m");

                    let mut input = String::new();

                    io::stdin().read_line(&mut input).unwrap();
                    //Take input and match to the zaddynum
                    //with klines
                    //println!("{:?}", &input);
                    input.pop();

                    let mut taddy = String::new();
                    let mut klines = zaddynum.split_whitespace();
                    loop {
                        //get first line, if any
                        let kv = klines.next().clone();
                        //assert EQ for Some
                        if kv != None {
                            //remove Result wrapper
                            kvc = kv.unwrap();
                            //match here

                            if input == kvc {
				taddy = klines.next().unwrap().to_string();
								
                            }
				continue
				}
			break
			}
                
                //println!("the taddy {:?}", &taddy);
                let goodtadd = taddy.replace('"', "");
                //println!("goodtadd  {:?}", &goodtadd);

                
                  println!("\x1b[48;5;58m\x1b[38;5;188mEnter Your Message to Sign\x1b[0m");

                    let mut input = String::new();

                    io::stdin().read_line(&mut input).unwrap();
					input.pop();
					
					
{
                {
                    //println!("G - getinfo ok!");

                    let s5 = String::from(" signmessage ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
			resend0.push_str(&goodtadd);
			resend0.push(' ');
			resend0.push('"');
                    resend0.push_str(&input);
			resend0.push('"');
			resend0.push(' ');

                    resend0.push_str(resend00000);

                    //println!("resend0 {}", &resend0);

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let mut k = fs::read_to_string(&sp2).unwrap();
                    k.pop();
                    println!();
			println!("\x1b[48;5;58m\x1b[38;5;188mSignature: \x1b[0m");

                    println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &k);
			println!();

			println!("\x1b[48;5;58m\x1b[38;5;188mSigning Address: \x1b[0m");
					
			println!("\x1b[48;5;52m\x1b[38;5;187m{}\x1b[0m", &goodtadd);
                    println!();

			println!("\x1b[48;5;58m\x1b[38;5;188mMessage: \x1b[0m");

			println!("\x1b[48;5;53m\x1b[38;5;187m{}\x1b[0m", &input);

                    
                    
                    
                    
                }
            }
                
            }
            
		
		
		
             "V" => {
                {
                   
println!("\x1b[48;5;58m\x1b[38;5;188mEnter the Signing Address \x1b[0m");

                    let mut input = String::new();

                    io::stdin().read_line(&mut input).unwrap();
                    input.pop();
                
                     		{
                                  {
                                        let s5 = String::from(" z_validateaddress ");

                                        let mut zadd = String::from(&input).clone();

                                        zadd.push(' ');

                                        let zaddy = zadd.replace('"', "");
                                        //println!("zaddy   {}", &zaddy);
                                        //println!("zaddybg {:?}", &zaddy);

                                        let mut resend0 = resend.clone();
                                        let resend0000 = s5.as_str();

                                        resend0.push_str(resend00);
                                        resend0.push_str(resend000);
                                        resend0.push_str(resend0000);
                                        resend0.push_str(zaddy.as_str());
                                        resend0.push_str(resend00000x);

                                        //println!("{}\n", &resend0);

                                        let mut var_1 = File::create(&rp).expect("no");
                                        var_1.write_all(&resend0.as_bytes()).expect("no");
                                    }

                                    let mut inotify = Inotify::init()
                                        .expect("Error while initializing inotify instance");
                                    inotify
                                        .add_watch(&rp2, WatchMask::CLOSE_WRITE)
                                        .expect("Failed to add file watch");

                                    let mut buffer = [0; 512];
                                    let events = inotify
                                        .read_events_blocking(&mut buffer)
                                        .expect("Error while reading events");
                                    #[allow(unused_variables)]
                                    for event in events {
                                        let k = fs::read_to_string(&rp2).unwrap();
                                        let mut klines = k.split_whitespace();
                                        loop {
                                            //get first line, if any
                                            let kv = klines.next().clone();
                                            //assert EQ for Some
                                            if kv != None {
                                                //remove Result wrapper
                                                kvc = kv.unwrap();

                                                match kvc {
                                                    "[" | "]" | "{" | "}" | "\\" | "}," | "],"
                                                    | "\"addresses\":" | "\"source\":" => {}
                                                    "\"seedfp\":" | "\"diversifier_index\":" => {
                                                        klines.next();
                                                    }
                                                    _ => (),
                                                }
                                                if kvc.starts_with("\"isvalid\"") {
                                                    let bewl = klines.next().clone();
                                                    bewl.unwrap();
                                                    //println!("bewl   {:?}", &bewl);

                                                    if bewl.expect("bewlfail").starts_with("false")
                                                    {
                                                        println!("\x1b[48;5;52m\x1b[38;5;187mThe Address is Invalid, Please Try Again\x1b[0m");
                                                        {
                                                            break;
                                                        }
                                                    }
                                                }
                                                
                                                
                                                continue;
                                            }
                                            break;
                                        }
                                        //println!("\x1b[48;5;23m\x1b[38;5;187m{:?}\x1b[0m", &k);
                                    }
                                }
                    
                    
                    println!("\x1b[48;5;58m\x1b[38;5;188mAddress is Valid\x1b[0m");
					
			let taddy = String::from(input).clone();
                    //taddy = input.clone();
                    println!();
                    println!("\x1b[48;5;58m\x1b[38;5;188mEnter the Signature \x1b[0m");

                    let mut signat = String::new();
                    io::stdin().read_line(&mut signat).unwrap();
			signat.pop();
			println!();
			println!("\x1b[48;5;58m\x1b[38;5;188mEnter the Signed Message \x1b[0m");

                    let mut emesgee = String::new();

                    io::stdin().read_line(&mut emesgee).unwrap();
                    emesgee.pop();
                    
			println!();
                    
                    let s5 = String::from(" verifymessage ");
                    let mut resend0 = resend.clone();
                    let resend0000 = s5.as_str();

                    resend0.push_str(resend00);
                    resend0.push_str(resend000);
                    resend0.push_str(resend0000);
			resend0.push_str(&taddy);
			resend0.push(' ');
			resend0.push_str(&signat);
			resend0.push(' ');
			resend0.push('"');
			resend0.push_str(&emesgee);
			resend0.push('"');
			resend0.push(' ');
                    resend0.push_str(resend00000);

                    //println!("{}\n", &resend0);

                    let mut var_1 = File::create(&sp).expect("no");
                    var_1.write_all(&resend0.as_bytes()).expect("no");
                }

                let mut inotify =
                    Inotify::init().expect("Error while initializing inotify instance");
                inotify
                    .add_watch(&sp2, WatchMask::CLOSE_WRITE)
                    .expect("Failed to add file watch");

                let mut buffer = [0; 512];
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                #[allow(unused_variables)]
                for event in events {
                    let k = fs::read_to_string(&sp2).unwrap();
                    
                    println!("\x1b[48;5;23m\x1b[38;5;187m{}\x1b[0m", &k);
					println!();
                }
            }
		
            "X" => {
                println!("\x1b[48;5;52m\x1b[38;5;187mExiting Now! Goodbye!\x1b[0m");
                process::exit(1);
            }

            _ => println!("\x1b[48;5;52m\x1b[38;5;187mInvalid, Please Try Again\x1b[0m"),
        }
    }
}
