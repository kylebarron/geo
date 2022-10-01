> Preface: I'm relatively new to Rust (and to the georust ecosystem), so feedback is welcome! Hopefully these ideas aren't totally insane :smile:.

# RFC: geo-types traits for storage-independent zero-copy access

## Motivation

**The core question of this RFC is: is it possible to define general traits for coordinate access that third party libraries could implement, so that `geo` algorithms could be used with other memory layouts?**

Can an implementation of traits trade standardized _storage_ for standardized _access_?

`geo-types` is the basis for geospatial data handling in Rust. This crate defines standardized structs for storing geospatial vector data. However, this is limiting in the sense that geodata can't be used until copied into these structs. There have been recent advances


I see it as: structs define standardized _storage_, while traits define standardized _access_. This seems extremely powerful because it can "future proof" the API to future memory layouts. If we define a flexible access

Structs: standardized storage
Traits: standardized access

As it stands, `geo` appears to be built

The flexibility of traits seems to be hugely beneficial here, because it enables different "memory backends" to make use of `geo` algorithms. In effect, `geo`'s default implementation would be the existing `geo` structs, but third parties could create alternative implementations that also work out of the box.

I believe this is worth discussion because it has the potential to increase adoption and interoperability and improve performance.

### Potential zero-copy memory formats

#### GeoArrow

[Apache Arrow](https://arrow.apache.org/) is a columnar data format designed for cross-language support and zero-copy access. [GeoArrow](https://github.com/geopandas/geo-arrow-spec) is an in-progress specification to extend Arrow to handle geospatial vector data. When using the arrow-native coordinate encoding, any coordinate of any geometry in the array can be accessed in constant time.


#### FlatGeobuf

FlatGeobuf is a performance-oriented vector geometry format. Until now, FlatGeobuf has been used exclusively as a _file format_, where language support for FlatGeobuf means loading the file to convert it into that language's native geometry type. But Flatbuffers, which FlatGeobuf is built upon, allows for zero-copy access. Thus, I don't see why FlatGeobuf couldn't also be used as a _memory format_.


#### GEOS?

TODO: reword this:

A GEOS trait implementation would be exciting because it could enable usage of both GEOS and pure-Rust algorithms without dual copies of the data. It would also support a system for incrementally moving to a pure-rust algorithm implementation, but that uses GEOS implementations for functions that `geo` has not yet implemented.

It might also enable a Rust program to interoperate seamlessly with other languages, say Python and Shapely, without copying the data (though TBH not entirely sure if this is possible).



## Trait Implementation

Now the question is: is it possible to implement this in `geo-types` in a way that is: backwards-compatible, performant, and maintainable?

https://users.rust-lang.org/t/supertraits-vs-generic-implementations/21266


### Partial geometry types?


### Ownership

I at first planned to use references in the traits, but ended up hitting
```
cannot return value referencing local variable
```
As I understand it, Arrow2 is designed to support copy-on-write semantics, so clones are really cheap and made things easier. This is certainly something to revisit.

### Reading


### Writing

TODO:

Writing to another format? I.e. reading from flatgeobuf but writing to geo types? Could there be a generic writer argument that defaults to the geo-types struct implementation?

## External Format Implementations

### GeoArrow

Currently there are two different geometry storage formats under discussion in GeoArrow: Well-known Binary and an Arrow-native encoding using nested lists.

1. Well-known Binary. This was the first geometry format defined by the spec, chosen for broad ecosystem support. However WKB
2. Arrow-native nested lists of coordinates. The Arrow format describes a storage format for nested lists. Essentially this stores the raw values in a single flat array, and then includes an array of indexes

With the arrow-native coordinate implementation, any coordinate could be accessed in constant time.

### FlatGeobuf

I'm less familiar with the internals of FlatGeobuf; in particular, does the header metadata always include the byte offsets of every geometry in the file? It must when the file includes a spatial index, otherwise it wouldn't know how to seek to specific areas of the file for random reads.

Even if the byte offsets of each individual feature weren't stored in the header metadata, a FlatGeobuf trait implementation could first scan the file to find the byte offsets of every feature. Then later coordinate access would likely be zero-copy and constant time.

### GEOS?

Does GEOS provide zero-copy access to its byte buffer? Judging from the [`get_coord_seq` method](https://docs.rs/geos/latest/geos/trait.Geom.html#tymethod.get_coord_seq), it looks like it might be possible to get a [GEOS Coordinate Sequence](https://docs.rs/geos/latest/geos/struct.CoordSeq.html) with no copy, though at this point a clone is used:

> Get the underlying geos `CoordSeq` object from the geometry

> Note: this clones the underlying `CoordSeq` to avoid double free (because `CoordSeq` handles the object ptr and the `CoordSeq` is still owned by the geos geometry) if this method’s performance becomes a bottleneck, feel free to open an issue, we could skip this clone with cleaner code.


## Backwards Compatibility

The addition of traits themselves would not allow implementing traits to be used out of the box. Every function that currently accepts a concrete type would need to be updated to accept any implementation of the trait instead.

However given that `geo-types` structs would implement the traits, I imagine that this update would preserve backwards compatibility.

## Arrow implementation details

Arrow2 also exposes `Scalar` types, like `StructScalar`, but I decided not to go with those because they're dynamically typed, and I didn't want to be doing dynamic dispatch on every row (though I have no idea whether that overhead is something worth optimizing).
