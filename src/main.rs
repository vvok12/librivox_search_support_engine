mod libgen_dump;

use libgen_dump::LibGenDump;

fn main() {
    let dump = LibGenDump::load();
    dump.extract();
    
    println!("dump is {:?}", dump.dump);
}
