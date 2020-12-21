# Lantmäteriet Shape to OSM converter

This is a converter between [`Lantmäteriet`] (The Swedish National Land Survey)
and the OSM format with the goal to get these free terrain maps into Garmin gps devices.

This project is a working prototype I use for my hiking maps.


## How to convert Lantmäteriet maps to garmin

Note, this is a quite technical process, no effort has been made to make it easy.


### Download the map
Download your maps from Lantmäteriet open data service:
https://www.lantmateriet.se/en/maps-and-geographic-information/geodataprodukter/produktlista/terrangkartan/

Click "Download product", create an account, then click the "Proceed to the download site - FTP". The map must be
downloaded in Shape file format.

### Download mkgmap
To convert from osm to gmapsupp, the garmin format, you will need mkgmap and the splitter provided by mkgmap.

Download mkgmap: https://www.mkgmap.org.uk/download/mkgmap.html
Download the splitter: https://www.mkgmap.org.uk/download/splitter.html

You'll need to unzip them, we need the jar files later on.

### Build this project
I don't provide any binaries, so you'll need to build it your self.

Linux/Mac:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ git clone git@github.com:Vadeen/lantmateriet_osm.git
$ cd lantmateriet_osm/
lantmateriet_osm $ cargo build --release
```

### Convert your map to o5m format
```
lantmateriet_osm $ ./target/release/cli ~/maps/gavleborg/terrang/21/ --output map.o5m
```
Where `~/maps/gavleborg/terrang/21/` is the path to the unzipped map you downloaded from lantmateriet.
This may take a while.

### Convert your o5m map to gmapsupp
```
 lantmateriet_osm $ mkdir mkgmap
 lantmateriet_osm $ cd mkgmap
 mkgmap $ java -jar ~/Downloads/mkgmap.jar --family-id=909 ../styles/typfile.txt
 mkgmap $ java -jar ~/Downloads/splitter.jar ../map.o5m
 mkgmap $ java -jar ~/Downloads/mkgmap.jar --gmapsupp --family-id=909 --style-file=../styles *.pbf *.typ
```
Where `~/Downloads/mkgmap.jar` and `~/Downloads/splitter.jar` are jars from the mkgmap project.

You now have a `gmapsupp.img` you can copy to a sd-card and put in your garmin gps device!

[`Lantmäteriet`]: https://en.wikipedia.org/wiki/Lantm%C3%A4teriet
