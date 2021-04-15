# Dutch roadmap

Implementation of Highway node routing. Using the Dutch road network as data.

https://www.rijkswaterstaat.nl/apps/geoservices/geodata/dmc/weggeg/geogegevens/shapefile/
https://www.rijkswaterstaat.nl/apps/geoservices/geodata/dmc/nwb-wegen/geogegevens/shapefile/Nederland_totaal/


https://www.rijkswaterstaat.nl/apps/geoservices/geodata/dmc/nwb-wegen/productinfo/beschrijvende_documentatie/

## Data
https://www.rijkswaterstaat.nl/apps/geoservices/geodata/dmc/nwb-wegen/geogegevens/shapefile/Nederland_totaal/01-12-2020/Wegvakken/
```
> data
    > Hectopunten
    > Wegvakken
```

## Current goals

- Visualize
- Junctions 
- Create network
    - rijrichting (RIJRICHTING)
    - juncties (JTE_ID_BEGIN -> JTE_ID_END)


## Layered Network:

[Highway node routing](http://algo2.iti.uni-karlsruhe.de/schultes/hwy/dynamic.pdf).
[Engineering Highway hierarchies](http://algo2.iti.kit.edu/documents/routeplanning/hhJournalSubmit.pdf)

A network consists of roads (wegvakken) and their connections (juncties).

- Roads can be one-way or two-way
- Roads should have their attributes
- Hectometerized roads have a Baanpostie_tov_Wol 
    - wol (wegoriëntatie lijn)
    - R#, M, L#
    - are probably connected to each other
- Would be nice if it could be streamed.

### Streaming of network?
What I think of streaming.

The aim is the create a layered network. A node in the network can lead to a higher layer. Higher layers have fewer nodes, and represent abstracted roads.

Assuming the network is used for pathfinding between two different points.
The search is started at these two points and will meet each other halfway at the same level.
> Conclusion: A region around the starting points at level 0 is loaded at start. 
> when ascending levels, region around the entry points of these levels are loaded when needed.

### Precomputing

A basic network needs to be created: Level 0.
Then multiple layers need to be created: 