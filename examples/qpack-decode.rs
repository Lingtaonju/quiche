// Copyright (C) 2019, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#[macro_use]
extern crate log;

use std::fs::File;

use std::io::prelude::*;

use quiche::h3::qpack;

const USAGE: &str = "Usage:
  qpack-decode [options] FILE
  qpack-decode -h | --help

Options:
  -h --help  Show this screen.
";

fn main() {
    env_logger::init();

    let args = docopt::Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    // TODO: parse params from file name.

    let mut file = File::open(args.get_str("FILE")).unwrap();

    let mut dec = qpack::Decoder::new();

    loop {
        let mut stream_id: [u8; 8] = [0; 8];
        let mut len: [u8; 4] = [0; 4];

        let _ = file.read(&mut stream_id).unwrap();
        let stream_id = u64::from_be_bytes(stream_id);

        let _ = file.read(&mut len).unwrap();
        let len = u32::from_be_bytes(len) as usize;

        let mut data = vec![0; len as usize];

        let data_len = file.read(&mut data).unwrap();

        if data_len == 0 {
            break;
        }

        debug!("Got stream={} len={}", stream_id, len);

        if stream_id == 0 {
            dec.control(&mut data[..len]).unwrap();
            continue;
        }

        for hdr in dec.decode(&mut data[..len], std::u64::MAX).unwrap() {
            println!("{}\t{}", hdr.name(), hdr.value());
        }

        println!();
    }
}
