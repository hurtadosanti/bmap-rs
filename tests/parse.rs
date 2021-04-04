use bmap::Bmap;

#[test]
fn parse() {
    let xml = include_str!("data/simple.bmap");
    let bmap = Bmap::from_xml(xml).unwrap();

    assert_eq!(4194304, bmap.image_size());
    assert_eq!(4096, bmap.block_size());
    assert_eq!(1024, bmap.blocks());
    assert_eq!(680, bmap.mapped_blocks());

    let mut block = 0;
    for range in bmap.block_map() {
        assert_eq!(block * 4096, range.offset());
        assert_eq!((block + 1) * 4096 , range.length());
        // TODO checksum
        block = if block == 0 { 8 } else { block * 4 };
    }
    assert_eq!(2048, block);
}