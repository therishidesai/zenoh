#[cfg(feature = "unstable")]
#[test]
fn pubsub() {
    use zenoh::{prelude::sync::*, sample_builder::SampleBuilderTrait};

    let zenoh = zenoh::open(Config::default()).res().unwrap();
    let _sub = zenoh
        .declare_subscriber("test/attachment")
        .callback(|sample| {
            println!(
                "{}",
                std::str::from_utf8(&sample.payload().contiguous()).unwrap()
            );
            for (k, v) in sample.attachment().unwrap() {
                assert!(k.iter().rev().zip(v.as_slice()).all(|(k, v)| k == v))
            }
        })
        .res()
        .unwrap();
    let publisher = zenoh.declare_publisher("test/attachment").res().unwrap();
    for i in 0..10 {
        let mut backer = [(
            [0; std::mem::size_of::<usize>()],
            [0; std::mem::size_of::<usize>()],
        ); 10];
        for (j, backer) in backer.iter_mut().enumerate() {
            *backer = ((i * 10 + j).to_le_bytes(), (i * 10 + j).to_be_bytes())
        }
        zenoh
            .put("test/attachment", "put")
            .with_attachment(
                backer
                    .iter()
                    .map(|b| (b.0.as_slice(), b.1.as_slice()))
                    .collect(),
            )
            .res()
            .unwrap();
        publisher
            .put("publisher")
            .with_attachment(
                backer
                    .iter()
                    .map(|b| (b.0.as_slice(), b.1.as_slice()))
                    .collect(),
            )
            .res()
            .unwrap();
    }
}
#[cfg(feature = "unstable")]
#[test]
fn queries() {
    use zenoh::{prelude::sync::*, sample::Attachment, sample_builder::SampleBuilderTrait};

    let zenoh = zenoh::open(Config::default()).res().unwrap();
    let _sub = zenoh
        .declare_queryable("test/attachment")
        .callback(|query| {
            println!(
                "{}",
                std::str::from_utf8(
                    &query
                        .value()
                        .map(|q| q.payload.contiguous())
                        .unwrap_or_default()
                )
                .unwrap()
            );
            let mut attachment = Attachment::new();
            for (k, v) in query.attachment().unwrap() {
                assert!(k.iter().rev().zip(v.as_slice()).all(|(k, v)| k == v));
                attachment.insert(&k, &k);
            }
            query
                .reply(
                    query.key_expr().clone(),
                    query.value().unwrap().payload.clone(),
                )
                .with_attachment(attachment)
                .res()
                .unwrap();
        })
        .res()
        .unwrap();
    for i in 0..10 {
        let mut backer = [(
            [0; std::mem::size_of::<usize>()],
            [0; std::mem::size_of::<usize>()],
        ); 10];
        for (j, backer) in backer.iter_mut().enumerate() {
            *backer = ((i * 10 + j).to_le_bytes(), (i * 10 + j).to_be_bytes())
        }
        let get = zenoh
            .get("test/attachment")
            .with_value("query")
            .with_attachment(
                backer
                    .iter()
                    .map(|b| (b.0.as_slice(), b.1.as_slice()))
                    .collect(),
            )
            .res()
            .unwrap();
        while let Ok(reply) = get.recv() {
            let response = reply.sample.as_ref().unwrap();
            for (k, v) in response.attachment().unwrap() {
                assert_eq!(k, v)
            }
        }
    }
}
