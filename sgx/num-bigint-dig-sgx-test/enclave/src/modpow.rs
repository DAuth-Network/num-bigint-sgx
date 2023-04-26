//extern crate num_bigint_dig as num_bigint;
//extern crate num_integer;
//extern crate num_traits;

static BIG_B: &'static str = "\
                              efac3c0a_0de55551_fee0bfe4_67fa017a_1a898fa1_6ca57cb1\
                              ca9e3248_cacc09a9_b99d6abc_38418d0f_82ae4238_d9a68832\
                              aadec7c1_ac5fed48_7a56a71b_67ac59d5_afb28022_20d9592d\
                              247c4efc_abbd9b75_586088ee_1dc00dc4_232a8e15_6e8191dd\
                              675b6ae0_c80f5164_752940bc_284b7cee_885c1e10_e495345b\
                              8fbe9cfd_e5233fe1_19459d0b_d64be53c_27de5a02_a829976b\
                              33096862_82dad291_bd38b6a9_be396646_ddaf8039_a2573c39\
                              1b14e8bc_2cb53e48_298c047e_d9879e9c_5a521076_f0e27df3\
                              990e1659_d3d8205b_6443ebc0_9918ebee_6764f668_9f2b2be3\
                              b59cbc76_d76d0dfc_d737c3ec_0ccf9c00_ad0554bf_17e776ad\
                              b4edf9cc_6ce540be_76229093_5c53893b";

static BIG_E: &'static str = "\
                              be0e6ea6_08746133_e0fbc1bf_82dba91e_e2b56231_a81888d2\
                              a833a1fc_f7ff002a_3c486a13_4f420bf3_a5435be9_1a5c8391\
                              774d6e6c_085d8357_b0c97d4d_2bb33f7c_34c68059_f78d2541\
                              eacc8832_426f1816_d3be001e_b69f9242_51c7708e_e10efe98\
                              449c9a4a_b55a0f23_9d797410_515da00d_3ea07970_4478a2ca\
                              c3d5043c_bd9be1b4_6dce479d_4302d344_84a939e6_0ab5ada7\
                              12ae34b2_30cc473c_9f8ee69d_2cac5970_29f5bf18_bc8203e4\
                              f3e895a2_13c94f1e_24c73d77_e517e801_53661fdd_a2ce9e47\
                              a73dd7f8_2f2adb1e_3f136bf7_8ae5f3b8_08730de1_a4eff678\
                              e77a06d0_19a522eb_cbefba2a_9caf7736_b157c5c6_2d192591\
                              17946850_2ddb1822_117b68a0_32f7db88";

// This modulus is the prime from the 2048-bit MODP DH group:
// https://tools.ietf.org/html/rfc3526#section-3
static BIG_M: &'static str = "\
                              FFFFFFFF_FFFFFFFF_C90FDAA2_2168C234_C4C6628B_80DC1CD1\
                              29024E08_8A67CC74_020BBEA6_3B139B22_514A0879_8E3404DD\
                              EF9519B3_CD3A431B_302B0A6D_F25F1437_4FE1356D_6D51C245\
                              E485B576_625E7EC6_F44C42E9_A637ED6B_0BFF5CB6_F406B7ED\
                              EE386BFB_5A899FA5_AE9F2411_7C4B1FE6_49286651_ECE45B3D\
                              C2007CB8_A163BF05_98DA4836_1C55D39A_69163FA8_FD24CF5F\
                              83655D23_DCA3AD96_1C62F356_208552BB_9ED52907_7096966D\
                              670C354E_4ABC9804_F1746C08_CA18217C_32905E46_2E36CE3B\
                              E39E772C_180E8603_9B2783A2_EC07A28F_B5C55DF0_6F4C52C9\
                              DE2BCBF6_95581718_3995497C_EA956AE5_15D22618_98FA0510\
                              15728E5A_8AACAA68_FFFFFFFF_FFFFFFFF";

static BIG_R: &'static str = "\
                              a1468311_6e56edc9_7a98228b_5e924776_0dd7836e_caabac13\
                              eda5373b_4752aa65_a1454850_40dc770e_30aa8675_6be7d3a8\
                              9d3085e4_da5155cf_b451ef62_54d0da61_cf2b2c87_f495e096\
                              055309f7_77802bbb_37271ba8_1313f1b5_075c75d1_024b6c77\
                              fdb56f17_b05bce61_e527ebfd_2ee86860_e9907066_edd526e7\
                              93d289bf_6726b293_41b0de24_eff82424_8dfd374b_4ec59542\
                              35ced2b2_6b195c90_10042ffb_8f58ce21_bc10ec42_64fda779\
                              d352d234_3d4eaea6_a86111ad_a37e9555_43ca78ce_2885bed7\
                              5a30d182_f1cf6834_dc5b6e27_1a41ac34_a2e91e11_33363ff0\
                              f88a7b04_900227c9_f6e6d06b_7856b4bb_4e354d61_060db6c8\
                              109c4735_6e7db425_7b5d74c7_0b709508";

pub mod biguint {
    use num_bigint::BigUint;
    use num_integer::Integer;
    use num_traits::Num;

    fn check_modpow<T: Into<BigUint>>(b: T, e: T, m: T, r: T) {
        let b: BigUint = b.into();
        let e: BigUint = e.into();
        let m: BigUint = m.into();
        let r: BigUint = r.into();

        assert_eq!(b.modpow(&e, &m), r);

        let even_m = &m << 1;
        let even_modpow = b.modpow(&e, &even_m);
        assert!(even_modpow < even_m);
        assert_eq!(even_modpow.mod_floor(&m), r);
    }

    //#[test]
    pub fn test_modpow_single() {
        check_modpow::<u32>(1, 0, 11, 1);
        check_modpow::<u32>(0, 15, 11, 0);
        check_modpow::<u32>(3, 7, 11, 9);
        check_modpow::<u32>(5, 117, 19, 1);
    }

    //#[test]
    pub fn test_modpow_big() {
        let b = BigUint::from_str_radix(super::BIG_B, 16).unwrap();
        let e = BigUint::from_str_radix(super::BIG_E, 16).unwrap();
        let m = BigUint::from_str_radix(super::BIG_M, 16).unwrap();
        let r = BigUint::from_str_radix(super::BIG_R, 16).unwrap();

        assert_eq!(b.modpow(&e, &m), r);

        let even_m = &m << 1;
        let even_modpow = b.modpow(&e, &even_m);
        assert!(even_modpow < even_m);
        assert_eq!(even_modpow % m, r);
    }
}

pub mod bigint {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::{Num, One, Signed, Zero};

    fn check_modpow<T: Into<BigInt>>(b: T, e: T, m: T, r: T) {
        fn check(b: &BigInt, e: &BigInt, m: &BigInt, r: &BigInt) {
            assert_eq!(&b.modpow(e, m), r, "{} ** {} (mod {}) != {}", b, e, m, r);

            let even_m = m << 1;
            let even_modpow = b.modpow(e, m);
            assert!(even_modpow.abs() < even_m.abs());
            assert_eq!(&even_modpow.mod_floor(&m), r);

            // the sign of the result follows the modulus like `mod_floor`, not `rem`
            assert_eq!(b.modpow(&BigInt::one(), m), b.mod_floor(m));
        }

        let b: BigInt = b.into();
        let e: BigInt = e.into();
        let m: BigInt = m.into();
        let r: BigInt = r.into();

        let neg_r = if r.is_zero() { BigInt::zero() } else { &m - &r };

        check(&b, &e, &m, &r);
        check(&-&b, &e, &m, &neg_r);
        check(&b, &e, &-&m, &-neg_r);
        check(&-b, &e, &-m, &-r);
    }

    //#[test]
    pub fn test_modpow() {
        check_modpow(1, 0, 11, 1);
        check_modpow(0, 15, 11, 0);
        check_modpow(3, 7, 11, 9);
        check_modpow(5, 117, 19, 1);
    }

    //#[test]
    pub fn test_modpow_big() {
        let b = BigInt::from_str_radix(super::BIG_B, 16).unwrap();
        let e = BigInt::from_str_radix(super::BIG_E, 16).unwrap();
        let m = BigInt::from_str_radix(super::BIG_M, 16).unwrap();
        let r = BigInt::from_str_radix(super::BIG_R, 16).unwrap();

        check_modpow(b, e, m, r);
    }

    //#[test]
    pub fn test_modpow_regressions() {
        let b = BigInt::from_str_radix("148481812629898028922243452517931778859", 10).unwrap();
        let e = BigInt::from_str_radix("164350981728398375121965168123110934994905698786703672972672898604245122565384632665320229592664080184165678526847345884979164161361870788392677397259783361669997458210059361790185938355877563593929478714825040836748164957314359093451997607971771611547549257793609195252376766141281726486479115003799144714128183665696714578467393814120227526482585239840445787567632354733951957802345386715871248483047881497956525866247944829277967526068822788411625449554600640196891378473177802068346095758817143275166167483977379612377628733368341719053305460274294511350837763626861228995578887259487962243311935062225921128133638312621437945519992818288453465098708801352407433629892022649558518668739019522985290473827609521712606998009402034024115903168144255768042129320865368708422779323482567497680433767646272900161006545330040104288222657772869086562080890990921279729840054238397946649195066553267786045462193245099006103846313224069126654860364743773661528709159184162331890246026968468075854895004614440562563403966571088898841287746465509970812429808338370105314308176865461375659159994405123891266709337612303317811367850519031055517613606059551539655265079163631883411122545335469009188561490562340100241858667397070522833201378945082367080303930737460085913589781310332202843084405789975523360515518724292604959852907884723834109301037010981810400469821789426150300238378910637164951422912400649144069581318797904885229033199330969917473583627360277003209342444115817442334409699908719803971129585552217720163472715197434582192016473415220486410376364470209613934593616891157837722052159595619082131329052418471178979749070210646603488411877432337686669419064020146677994675645890784629031645629931044021516402055317993934721616608563696148934577409311708245083906757791103637690292510165547063401385527997056954121665477882557077292209554459496917982492339487229688141530763150692622623272795444529846010384212201717448354354471390313394944641420537729794077616571212880759114553942399699567974275501943806450519656537154820877876153351290438129999404025460264150569923247671803347738438612339550712022372942743592714754306573808282605717032033275819875343175553669801881988323385785362441046181380541210414863721994233865976156241919018538304838096986853747426670289401018164419516853632768820383362107004227976035106131118469545950224360356987548199429365053752748081699631233511074051178296391097574448304825464270687816512317401375349484673480301886627859649660082713607771750803606946084699744123429385446074215481141415144959640495628130348624468766836381762042510936781300722486115488063326027291451024039766392335908184086768228847596527550616790205674080213548408229763037392340969472676595602270556401071384048020604327909934501565252293793797207146346237353181180650897635078348651682864682545684947006709194254396944396423192353225297735761017692031892010530886940052190428897070608145423884897961731293039683402223509642109932203493243263084842325764879968428129677259371460792715707895715737224078949761353247616162380061037191950050487463467812528656164400320234629304223345519745970775250821617679739209663950777506253481666213838435163785552701719451996405973898572946407314707644934778581322359906640869284294008054618184044944168077716469330553710607919475086198736822731411106767940517513117067340920713956504198958381975020396473042592923524697202199789614266606522684257572874995317955495218327193196753831545046930031947998147927851349772167106931459070942284239583716608592952560598540897287777232908661009153651601604269678247020505664949407134130455848520117547622541906732872100446640944626050064890054058389328759836051637590769709964190451361835977224657258154841427245128343914641758799690579706226595910843566133142399343803084018750765024209423758381252111340434598457630441263172401042037129339512480216561859237485485179316642715257798126649381183656694101241603952597449444909120197153522721490913694897355020355204849405171656334281390879810015575579384526915097294240353667407269558059437125234100571601655058663167086384390197946929355280399788013937609353436082888066756321802952215535926645873241527518339318793147919520660517850111074815282027596328391301608794376252865467079832695530445147672411416273240414437731626856569247968458705303971664831520406276181645211389639682423522154589153857862875820332977990620041874779912534487054875146227333182840793848696822083435299675425290586091746886253719376517376476192979085056002906185495241899325295106611248170270201999129505310823968875198192620688215223646536583263434182903541968689220812272083266201543646730605107614994880972852396541610915266940027186999857045792946342509904360006232236385410875003200003710271753907829437379680408328394065483143578396887566566005407578049525700830381650375391369588237592292574983213859202661428243174328514293270384618273277544583385984487503069129233267053646486572518591625604138011396668605361469032350603582888850778247186965442140444404195860308517896349031637246734945680372839546785278479593824257340527474351027861885392290231378838449725230480694930523008277023154555844343297155030642104598551668450374495246522115798459878558197309686627849547255201282303229719213831030170763906878819183484135468001583820625287461250511735890029467608943110965462084671268661889450052241131060465481175469285658052200025128774690538495390259618074262588140655147228805545049873691855054152255770202829440165503982450624005234795768600340087204677562163377727571121882970611075470502776106556822809472703656347295866746124605204135016829660149143986183668195830353797312832360933413687525614207287013489308257746670458517988651073504753625204959605466094771204209964965541710217080544310915210188893650613899394914870323514246198819988568145291383437326992546113583363899966403356219181683992559908962490561664206795233711792893981773521595699751343042228013932508198601083950252622077065636311291522313149517311977225604486651761581102311245044441267458710690116324603014332726884757767675055064026627538150429094284462465936700805774875213555790247688992265287670483674835359537320362066496631237532066434198486983108134077101914489258787273151097982046578991716153170761170565232354349286283062140146535745514689289849445004028675192230268395088320480288024809248753472669586551981632827331574644655254527360946541171694009094534525965788372249561700117566619377612607784622980162705615061563640861523841144853582789547670660998574405213291156583471422472737024713335911128552519494928220447015458117896038528532736249462994191693731924310038783792155090734024701098844694228302557144109939199656035117618815047127207778455632893493029828318966843557912588160243215378697136562712636447184586542592233327626401267509598734844522240759857137220876909047828060336559766882884056292087298828311843464955622703622199634508141793685859364925637793777682016420351323859848010606623465396948369240631276472572232028281297280828901171208847997385162695745049130595743829108979357004406444046586221335176301152731273434672154904521807542045622667736615188377717166157412872030872463207532820694823277895456126315590200107172399501418780662873848899945555090581629304192293210512166806174241905318439297836572429492555945848426574436820804276431938356677265173902567516974372557048005931467724243514313618168555104937201562664655492712786862271369422893595407516698240054274482308875406045004002766861865486805088644548951780356707149274987632240062289750700451450662820311466431667428284851948473650031507800214161385241719742922217838238400711647126794748806174836882883940057722793324676878558204321525826413953232881576775548034368728022937081394740112688238217301103514009488899854828922756909485766733545511482058947421794810707669182386862066862732498731756283370966668219295546440670559968588114589890542419364782206557895743456359437472986220063749869820539379941257862675002615164658188357798105487865311641962685185181852938728255920848800712212039448123892624090215049822390599899936382039234469555489723925532717532999939422880433214314964121376053233983116473767598132603437846016981354171586132813499569707397461677048841971510919245413813114311724904074380969238678296194626059869912965335551790872571048158053224153865323110852167314493276323090198653641172199104636190926118630451555110729692658524817414978729750372126722731500043191633465954497667185630804110549483087599351274519782412871406368481080021704006725857372058573865218993668019024024208275108900357152765961057292140846117515718034378698161351291158047506515803497515956519959532767226029329927621693742244108404900014394147196418357662166818493187358701629972155595893579939701750218259082071634738306600756059735100899792211287340114305538026232183669968800789674455430940867429885716527243860641947991536204288665082058524860060114668806790606990321001884609501157240809649183919063321158567912329113896832740889245598107417790596778129724526582814454491169202690995589063942376974544174226050004223359926067502761530455570619313444306185650028516198954161354984900397079331598983284892569733533525469198148807066556718513506092297233405158834932205982055789299794670101612866721906061130211546399655425605752494047788017152677453616235600992059935426874231748721437691974116227927409158423042936833609068849541189", 10).unwrap();
        let m = BigInt::from_str_radix("243440964008985994185807471607210276717", 10).unwrap();
        let r = BigInt::from_str_radix("138995801145388806366366393471481216294", 10).unwrap();

        check_modpow(b, e, m, r);
    }
}