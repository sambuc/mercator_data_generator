# Mercator Test data generator

Tool to generate test data for Mercator, a spatial index.

## Mercator: Spatial Index

**Mercator** is a spatial *volumetric* index for the [Human Brain Project](http://www.humanbrainproject.eu). It is a component of the [Knowledge Graph](http://www.humanbrainproject.eu/en/explore-the-brain/search/) service, which  provides the spatial anchoring for the metadata registered as well as processes the volumetric queries.

It is build on top of the Iron Sea database toolkit.

## Iron Sea: Database Toolkit

**Iron Sea** provides a set of database engine bricks, which can be combined and applied on arbitrary data structures.

Unlike a traditional database, it does not assume a specific physical structure for the tables nor the records, but relies on the developper to provide a set of extractor functions which are used by the specific indices provided.

This enables the index implementations to be agnostic from the underlying data structure, and re-used.

## Requirements

### Software

 * Rust: https://www.rust-lang.org

Checkout the dependencies in the parent folder:

 * mercator_db – https://github.com/epfl-dias/mercator_db
 * ironsea_index – https://github.com/epfl-dias/ironsea_index
 * ironsea_index_hashmap – https://github.com/epfl-dias/ironsea_index_hashmap
 * ironsea_index_sfc_dbc – https://github.com/epfl-dias/ironsea_index_sfc_dbc

## Quick start

This tool takes as arguments a list of number of features to generate.
One thousand (1000) positions are generated per feature. For each number
provided a dataset will be generated under:
 * `number`k.spaces.json -- The reference spaces for that dataset
 * `number`k.objects.json -- The objects randomly generated (`number` features, each with 1000 positions.)

You can also specify a factor value, which will generate for each feature `factor` times features sharing the same 1000 positions.

For example:

```sh
# Generate 3 datasets, with 1k, 10k, 100k, random positions and
#  1, 10 & 100 features ids.
cargo run --release -- 1 10 100

# Generate 3 datasets, with 1k, 10k, 100k, random positions and
#  2, 20, 200 features ids.
cargo run --release -- 1 10 100 -f 2
```

## Installation

To install the software on the system, after checking out the
dependencies you can use:

```sh
cargo install --path .
```

Then in any folder you can then use:

```sh
mercator_data_generator 1 10 100
# or
mercator_data_generator 1 10 100 -f 2
```

## Acknowledgements

This open source software code was developed in part or in whole in the
Human Brain Project, funded from the European Union’s Horizon 2020
Framework Programme for Research and Innovation under the Specific Grant
Agreement No. 785907 (Human Brain Project SGA2).
