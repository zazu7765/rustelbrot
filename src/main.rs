use std::{env, thread::scope};

use util::{img::{map_px_to_pt, render, write}, parse::{parse_complex, parse_pair}};



mod util;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!(
            "Usage: {} <file> <PIXELSxPIXELS> <upper_left> <lower_right> <threads>",
            args[0]
        );
        eprintln!("Example args: test.png 4000x3000 -1.20,0.35 -1,0.20 32");
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
    .unwrap_or_else(||{
        eprintln!("Bad image dimensions");
        std::process::exit(1);
    }
    );

    let ul = parse_complex(&args[3])
    .unwrap_or_else(||{
        eprintln!("Bad Upper Left Corner Point");
        std::process::exit(1);
    }
    );

    let lr = parse_complex(&args[4])
    .unwrap_or_else(||{
        eprintln!("Bad Lower Right Corner Point");
        std::process::exit(1);
    }
    );

    let threads = args[5].parse::<usize>()
    .unwrap_or(1);    

    let mut px = vec![0; bounds.0 * bounds.1];

    let rows_per_band = bounds.1 / threads + 1;

    if threads > 1
    {
        let bands: Vec<&mut[u8]> = px.chunks_mut(rows_per_band * bounds.0).collect();

        scope(|spawn|{
            for(i, band) in bands.into_iter().enumerate(){
                let top = rows_per_band * i;
                let h = band.len()/bounds.0;
                let band_bounds = (bounds.0, h);
                let band_ul = map_px_to_pt(bounds, (0, top), ul, lr);
                let band_lr = map_px_to_pt(bounds, (bounds.0, top + h), ul, lr);

                spawn.spawn(move ||{
                    render(band, band_bounds, band_ul, band_lr);
                });
            }
        })
    }
    else{
        render(&mut px, bounds, ul, lr);
    }

    write(&args[1], &px, bounds)
    .expect("Error Writing File");

}
