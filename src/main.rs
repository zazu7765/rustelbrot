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
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
    .expect("Bad Image Dimensions");

    let ul = parse_complex(&args[3])
    .expect("Bad Upper Left Corner Point");

    let lr = parse_complex(&args[4])
    .expect("Bad Lower Right Corner Point");

    let mut px = vec![0; bounds.0 * bounds.1];

    let threads = args[5].parse::<usize>()
    .expect("Bad Thread Count");

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
