//fn pool()
use std::thread;
use std::sync::mpsc::*;  


type index = usize;

fn worker<F>(fun:F, rx:Receiver<Option<(index,(i32,i32))>>, tx:Sender<(index,i32)>)
where F:Fn((index,(i32,i32))) -> (index,i32)
{
    while let Ok(msg) = rx.recv(){
        if let Some(msg) = msg{
            let res = fun(msg);
            tx.send(res);
        }
        else {break}
    }
}

fn main()
{
    let work = vec![
        (1,1),
        (100,9),
        (7,42),
        (2,2),
        (23,4711),
        (123,456),
        (2934,123123)
    ];

    let (tc, rp) = channel();

    let mut tps:Vec<Sender<Option<(index,(i32,i32))>>> = Vec::new();

    // init workers
    for _ in 0..5 {

        let tc = tc.clone();
        let (tp, rc) = channel();

        let handle = thread::spawn(move || {
            worker(
                |(i,(a,b))| (i, a + b),
                rc,
                tc
                )
        }); 

        tps.push(tp);

    }

    // send work
    for (tp, (i,work_tup)) in tps.iter().cycle().zip(work.iter().enumerate()){
        tp.send(Some((i,work_tup.clone())));
    }

    //let mut ress:Vec<i32> = Vec::with_capacity(work.len());
    let mut ress = (0..work.len() as i32).collect::<Vec<i32>>();

    for work in work{
        let (i, r) = rp.recv().unwrap();
        ress[i] = r;
    }

    println!("{:?}", ress);

}
