extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system h3
    // shared library.
    println!("cargo:rustc-link-lib=h3.1");
    println!("cargo:rustc-link-search=native=/usr/local/lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .whitelist_type("BaseCellData")
        .whitelist_type("BaseCellOrient")
        .whitelist_type("BBox")
        .whitelist_type("CoordIJK")
        .whitelist_type("FaceIJK")
        .whitelist_type("FaceOrientIJK")
        .whitelist_type("GeoBoundary")
        .whitelist_type("GeoCoord")
        .whitelist_type("Geofence")
        .whitelist_type("GeoMultiPolygon")
        .whitelist_type("GeoPolygon")
        .whitelist_type("H3IndexFat")
        .whitelist_type("LinkedGeoCoord")
        .whitelist_type("LinkedGeoLoop")
        .whitelist_type("LinkedGeoPolygon")
        .whitelist_type("TestOutput")
        .whitelist_type("Vec2d")
        .whitelist_type("VertexGraph")
        .whitelist_type("VertexNode")
        .whitelist_function("geoToH3")
        .whitelist_function("h3ToGeo")
        .whitelist_function("h3ToGeoBoundary")
        .whitelist_function("kRing")
        .whitelist_function("maxKringSize")
        .whitelist_function("kRingDistances")
        .whitelist_function("hexRange")
        .whitelist_function("hexRangeDistances")
        .whitelist_function("hexRanges")
        .whitelist_function("hexRing")
        .whitelist_function("polyfill")
        .whitelist_function("maxPolyfillSize")
        .whitelist_function("h3SetToLinkedGeo")
        .whitelist_function("destroyLinkedPolygon")
        .whitelist_function("degsToRads")
        .whitelist_function("radsToDegs")
        .whitelist_function("hexAreaKm2")
        .whitelist_function("hexAreaM2")
        .whitelist_function("edgeLengthKm")
        .whitelist_function("edgeLengthM")
        .whitelist_function("numHexagons")
        .whitelist_function("h3GetResolution")
        .whitelist_function("h3GetBaseCell")
        .whitelist_function("stringToH3")
        .whitelist_function("h3ToString")
        .whitelist_function("h3IsValid")
        .whitelist_function("h3ToParent")
        .whitelist_function("h3ToChildren")
        .whitelist_function("maxH3ToChildrenSize")
        .whitelist_function("compact")
        .whitelist_function("uncompact")
        .whitelist_function("maxUncompactSize")
        .whitelist_function("h3IsResClassIII")
        .whitelist_function("h3IsPentagon")
        .whitelist_function("h3IndexesAreNeighbors")
        .whitelist_function("getH3UnidirectionalEdge")
        .whitelist_function("h3UnidirectionalEdgeIsValid")
        .whitelist_function("getOriginH3IndexFromUnidirectionalEdge")
        .whitelist_function("getDestinationH3IndexFromUnidirectionalEdge")
        .whitelist_function("getH3IndexesFromUnidirectionalEdge")
        .whitelist_function("getH3UnidirectionalEdgesFromHexagon")
        .whitelist_function("getH3UnidirectionalEdgeBoundary")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}