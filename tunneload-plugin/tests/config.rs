mod config {
    use tunneload_plugin::Config;

    #[test]
    fn string_config() {
        let original: String = "some random test data".to_owned();

        assert_eq!(original.len(), Config::len(&original));

        let serialized = Config::serialize_data(&original);

        let parsed: String = Config::deserialize_data(&serialized).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn vec_config() {
        let original: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(original.len(), Config::len(&original));

        let serialized = Config::serialize_data(&original);

        let parsed: Vec<u8> = Config::deserialize_data(&serialized).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn struct_config() {
        #[derive(Debug, Config, PartialEq)]
        struct Tmp {
            name: String,
            content: Vec<u8>,
        }

        let original = Tmp {
            name: "test".to_owned(),
            content: vec![1, 2, 3, 4],
        };

        assert_eq!(
            (original.name.len() + 4 + original.content.len() + 4),
            Config::len(&original)
        );

        let serialized = Config::serialize_data(&original);

        let parsed: Tmp = Config::deserialize_data(&serialized).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn enum_config() {
        #[derive(Debug, PartialEq, Config)]
        enum Tmp {
            First,
            Second(String),
        }

        assert_eq!(1, Config::len(&Tmp::First));
        assert_eq!(vec![0], Config::serialize_data(&Tmp::First));

        let original = Tmp::Second("content".to_owned());

        assert_eq!((1 + 4 + "content".len()), Config::len(&original));

        let serialized = Config::serialize_data(&original);

        let parsed: Tmp = Config::deserialize_data(&serialized).unwrap();

        assert_eq!(original, parsed);
    }
}
