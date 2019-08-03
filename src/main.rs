use geojson::GeoJson;

mod geojsonarea;

fn main() {
    // Data from Puy-de-dôme, France : https://cadastre.data.gouv.fr/datasets/cadastre-etalab
    let geojson_str = r#"
    {"type":"FeatureCollection","features":[
    {"type":"Feature","id":"630030000A0118","geometry":{"type":"Polygon","coordinates":[[[3.6893315,45.5618765],[3.6892577,45.5619593],[3.6892527,45.5619666],[3.6890446,45.5622785],[3.6889387,45.5624375],[3.6889385,45.5624378],[3.6888923,45.5625368],[3.6888775,45.562703],[3.6882533,45.562456],[3.6865789,45.5617934],[3.6865623,45.5617869],[3.6864108,45.5618078],[3.6862557,45.5618412],[3.6861419,45.5618892],[3.6856219,45.5615518],[3.6860758,45.5611041],[3.6861313,45.5610678],[3.6861738,45.5610523],[3.686356,45.5610103],[3.6864777,45.5609664],[3.6865261,45.5609562],[3.6866101,45.5609648],[3.6867865,45.5610024],[3.6868376,45.5610278],[3.6869211,45.5610899],[3.6870448,45.5612856],[3.6871782,45.5615635],[3.687245,45.5616756],[3.6878707,45.561575],[3.6884364,45.5614478],[3.688594,45.5614293],[3.6888449,45.5614099],[3.6896917,45.5611989],[3.6898564,45.5613857],[3.6894084,45.561769],[3.6893634,45.5618407],[3.6893315,45.5618765]]]},"properties":{"id":"630030000A0118","commune":"63003","prefixe":"000","section":"A","numero":"118","contenance":26510,"arpente":false,"created":"2010-02-05","updated":"2016-06-15"}},
    {"type":"Feature","id":"630030000A0128","geometry":{"type":"Polygon","coordinates":[[[3.6861419,45.5618892],[3.6862557,45.5618412],[3.6866715,45.5626117],[3.6868779,45.5629943],[3.6866859,45.5634235],[3.6866727,45.563447],[3.6866122,45.5635003],[3.6855597,45.5634707],[3.6854931,45.5626627],[3.6853153,45.562364],[3.6854003,45.5623417],[3.6857482,45.5622501],[3.6857304,45.5621557],[3.6861419,45.5618892]]]},"properties":{"id":"630030000A0128","commune":"63003","prefixe":"000","section":"A","numero":"128","contenance":14810,"arpente":false,"created":"2010-02-05","updated":"2016-06-15"}},
    {"type":"Feature","id":"630030000A0101","geometry":{"type":"Polygon","coordinates":[[[3.6917414,45.561256],[3.6926246,45.5612771],[3.6927138,45.5612792],[3.6922236,45.5618345],[3.6930932,45.5619863],[3.6926396,45.5622866],[3.6923162,45.5626711],[3.6921092,45.5628691],[3.6916837,45.5631612],[3.6916224,45.5631057],[3.6915833,45.5630703],[3.691482,45.5629784],[3.6913222,45.5630669],[3.6912572,45.5631031],[3.691273,45.5631152],[3.6912422,45.5631334],[3.6913826,45.5632587],[3.6914029,45.5632768],[3.6913445,45.5632788],[3.6912525,45.5632649],[3.6911679,45.563233],[3.691089,45.5632728],[3.6906917,45.5628738],[3.690679,45.5628383],[3.6905322,45.562475],[3.6906715,45.5624305],[3.6907774,45.5623786],[3.6909564,45.5622482],[3.6911376,45.5620885],[3.6912155,45.5619985],[3.6913172,45.5617924],[3.6914038,45.5615871],[3.69142,45.5615335],[3.6914129,45.5612481],[3.6917414,45.561256]]]},"properties":{"id":"630030000A0101","commune":"63003","prefixe":"000","section":"A","numero":"101","contenance":23140,"arpente":false,"created":"2010-02-05","updated":"2016-06-15"}}
    ]}
    "#;

    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    process_geojson(geojson)
}

fn process_geojson(gj: GeoJson) {
    match gj {
        GeoJson::FeatureCollection(collection) => {
            for feat in collection.features {
                if let Some(geometry) = feat.geometry {
                    println!("Area: {}",geojsonarea::geometry(geometry))
                }
            }
        },
        GeoJson::Feature(feature) => {
            if let Some(geometry) = feature.geometry {
                println!("Area: {}",geojsonarea::geometry(geometry))
            }
        },
        GeoJson::Geometry(geometry) => {
            println!("Area: {}",geojsonarea::geometry(geometry))
        }
    }
}