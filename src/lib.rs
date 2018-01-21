#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod test {
    use *;
    use std::os::raw::c_int;

    #[test]
    fn test_geo_to_h3() {
        unsafe {
            let input = GeoCoord {
                lat: degsToRads(95.12345),
                lon: degsToRads(-21.12345),
            };
            let res: c_int = 5;
            let output = geoToH3(&input, res);
            assert_eq!(output, 599073528557338623);
            assert_eq!(format!("{:X}", output), "85056333FFFFFFF")
        }
    }
}