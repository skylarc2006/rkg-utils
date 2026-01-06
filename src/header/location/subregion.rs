#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subregion {
    Japan(JapanSubregions),
    Antarctica(AntarcticaSubregions),
    CaribbeanNetherlands(CaribbeanNetherlandsSubregions),
    FalklandIslands(FalklandIslandsSubregions),
    Scotland(ScotlandSubregions),
    Wales(WalesSubregions),
    SintMaarten(SintMaartenSubregions),
    Anguilla(AnguillaSubregions),
    AntiguaAndBarbuda(AntiguaAndBarbudaSubregions),
    Argentina(ArgentinaSubregions),
    Aruba(ArubaSubregions),
    Bahamas(BahamasSubregions),
    Barbados(BarbadosSubregions),
    Belize(BelizeSubregions),
    Bolivia(BoliviaSubregions),
    Brazil(BrazilSubregions),
    BritishVirginIslands(BritishVirginIslandsSubregions),
    Canada(CanadaSubregions),
    CaymanIslands(CaymanIslandsSubregions),
    Chile(ChileSubregions),
    Colombia(ColombiaSubregions),
    CostaRica(CostaRicaSubregions),
    Dominica(DominicaSubregions),
    DominicanRepublic(DominicanRepublicSubregions),
    Ecuador(EcuadorSubregions),
    ElSalvador(ElSalvadorSubregions),
    FrenchGuiana(FrenchGuianaSubregions),
    Grenada(GrenadaSubregions),
    Guadeloupe(GuadeloupeSubregions),
    Guatemala(GuatemalaSubregions),
    Guyana(GuyanaSubregions),
    Haiti(HaitiSubregions),
    Honduras(HondurasSubregions),
    Jamaica(JamaicaSubregions),
    Martinique(MartiniqueSubregions),
    Mexico(MexicoSubregions),
    Montserrat(MontserratSubregions),
    Curacao(CuracaoSubregions),
    Nicaragua(NicaraguaSubregions),
    Panama(PanamaSubregions),
    Paraguay(ParaguaySubregions),
    Peru(PeruSubregions),
    StKittsAndNevis(StKittsAndNevisSubregions),
    StLucia(StLuciaSubregions),
    StVincentAndTheGrenadines(StVincentAndTheGrenadinesSubregions),
    Suriname(SurinameSubregions),
    TrinidadAndTobago(TrinidadAndTobagoSubregions),
    TurksAndCaicosIslands(TurksAndCaicosIslandsSubregions),
    UnitedStates(UnitedStatesSubregions),
    Uruguay(UruguaySubregions),
    USVirginIslands(USVirginIslandsSubregions),
    Venezuela(VenezuelaSubregions),
    Armenia(ArmeniaSubregions),
    Belarus(BelarusSubregions),
    Georgia(GeorgiaSubregions),
    Kosovo(KosovoSubregions),
    Abkhazia(AbkhaziaSubregions),
    Catalonia(CataloniaSubregions),
    England(EnglandSubregions),
    SouthOssetia(SouthOssetiaSubregions),
    Bougainville(BougainvilleSubregions),
    Aland(AlandSubregions),
    FaroeIslands(FaroeIslandsSubregions),
    Albania(AlbaniaSubregions),
    Australia(AustraliaSubregions),
    Austria(AustriaSubregions),
    Belgium(BelgiumSubregions),
    BosniaAndHerzegovina(BosniaAndHerzegovinaSubregions),
    Botswana(BotswanaSubregions),
    Bulgaria(BulgariaSubregions),
    Croatia(CroatiaSubregions),
    Cyprus(CyprusSubregions),
    Czechia(CzechiaSubregions),
    Denmark(DenmarkSubregions),
    Estonia(EstoniaSubregions),
    Finland(FinlandSubregions),
    France(FranceSubregions),
    Germany(GermanySubregions),
    Greece(GreeceSubregions),
    Hungary(HungarySubregions),
    Iceland(IcelandSubregions),
    Ireland(IrelandSubregions),
    Italy(ItalySubregions),
    Latvia(LatviaSubregions),
    Lesotho(LesothoSubregions),
    Liechtenstein(LiechtensteinSubregions),
    Lithuania(LithuaniaSubregions),
    Luxembourg(LuxembourgSubregions),
    NorthMacedonia(NorthMacedoniaSubregions),
    Malta(MaltaSubregions),
    Montenegro(MontenegroSubregions),
    Mozambique(MozambiqueSubregions),
    Namibia(NamibiaSubregions),
    Netherlands(NetherlandsSubregions),
    NewZealand(NewZealandSubregions),
    Norway(NorwaySubregions),
    Poland(PolandSubregions),
    Portugal(PortugalSubregions),
    Romania(RomaniaSubregions),
    Russia(RussiaSubregions),
    Serbia(SerbiaSubregions),
    Slovakia(SlovakiaSubregions),
    Slovenia(SloveniaSubregions),
    SouthAfrica(SouthAfricaSubregions),
    Spain(SpainSubregions),
    Eswatini(EswatiniSubregions),
    Sweden(SwedenSubregions),
    Switzerland(SwitzerlandSubregions),
    Turkey(TurkeySubregions),
    UnitedKingdom(UnitedKingdomSubregions),
    Zambia(ZambiaSubregions),
    Zimbabwe(ZimbabweSubregions),
    Azerbaijan(AzerbaijanSubregions),
    Mauritania(MauritaniaSubregions),
    Mali(MaliSubregions),
    Niger(NigerSubregions),
    Chad(ChadSubregions),
    Sudan(SudanSubregions),
    Eritrea(EritreaSubregions),
    Djibouti(DjiboutiSubregions),
    Somalia(SomaliaSubregions),
    Andorra(AndorraSubregions),
    Gibraltar(GibraltarSubregions),
    Guernsey(GuernseySubregions),
    IsleOfMan(IsleOfManSubregions),
    Jersey(JerseySubregions),
    Monaco(MonacoSubregions),
    Taiwan(TaiwanSubregions),
    Cambodia(CambodiaSubregions),
    Laos(LaosSubregions),
    Mongolia(MongoliaSubregions),
    Myanmar(MyanmarSubregions),
    Nepal(NepalSubregions),
    Vietnam(VietnamSubregions),
    NorthKorea(NorthKoreaSubregions),
    SouthKorea(SouthKoreaSubregions),
    Bangladesh(BangladeshSubregions),
    Bhutan(BhutanSubregions),
    Brunei(BruneiSubregions),
    Maldives(MaldivesSubregions),
    SriLanka(SriLankaSubregions),
    TimorLeste(TimorLesteSubregions),
    BritishIndianOceanTerritory(BritishIndianOceanTerritorySubregions),
    HongKong(HongKongSubregions),
    Macao(MacaoSubregions),
    CookIslands(CookIslandsSubregions),
    Niue(NiueSubregions),
    NorfolkIsland(NorfolkIslandSubregions),
    NorthernMarianaIslands(NorthernMarianaIslandsSubregions),
    AmericanSamoa(AmericanSamoaSubregions),
    Guam(GuamSubregions),
    Indonesia(IndonesiaSubregions),
    Singapore(SingaporeSubregions),
    Thailand(ThailandSubregions),
    Philippines(PhilippinesSubregions),
    Malaysia(MalaysiaSubregions),
    SaintBarthelemy(SaintBarthelemySubregions),
    SaintMartin(SaintMartinSubregions),
    SaintPierreAndMiquelon(SaintPierreAndMiquelonSubregions),
    China(ChinaSubregions),
    Afghanistan(AfghanistanSubregions),
    Kazakhstan(KazakhstanSubregions),
    Kyrgyzstan(KyrgyzstanSubregions),
    Pakistan(PakistanSubregions),
    Tajikistan(TajikistanSubregions),
    Turkmenistan(TurkmenistanSubregions),
    Uzbekistan(UzbekistanSubregions),
    UAE(UAESubregions),
    India(IndiaSubregions),
    Egypt(EgyptSubregions),
    Oman(OmanSubregions),
    Qatar(QatarSubregions),
    Kuwait(KuwaitSubregions),
    SaudiArabia(SaudiArabiaSubregions),
    Syria(SyriaSubregions),
    Bahrain(BahrainSubregions),
    Jordan(JordanSubregions),
    Iran(IranSubregions),
    Iraq(IraqSubregions),
    Israel(IsraelSubregions),
    Lebanon(LebanonSubregions),
    StateOfPalestine(StateOfPalestineSubregions),
    Yemen(YemenSubregions),
    SanMarino(SanMarinoSubregions),
    VaticanCity(VaticanCitySubregions),
    Bermuda(BermudaSubregions),
    FrenchPolynesia(FrenchPolynesiaSubregions),
    Reunion(ReunionSubregions),
    Mayotte(MayotteSubregions),
    NewCaledonia(NewCaledoniaSubregions),
    WallisAndFutuna(WallisAndFutunaSubregions),
    Nigeria(NigeriaSubregions),
    Angola(AngolaSubregions),
    Ghana(GhanaSubregions),
    Togo(TogoSubregions),
    Benin(BeninSubregions),
    BurkinaFaso(BurkinaFasoSubregions),
    CoteDIvoire(CoteDIvoireSubregions),
    Liberia(LiberiaSubregions),
    SierraLeone(SierraLeoneSubregions),
    Guinea(GuineaSubregions),
    GuineaBissau(GuineaBissauSubregions),
    Senegal(SenegalSubregions),
    TheGambia(TheGambiaSubregions),
    CapeVerde(CapeVerdeSubregions),
    SaintHelenaAscensionAndTristanDaCunha(SaintHelenaAscensionAndTristanDaCunhaSubregions),
    Moldova(MoldovaSubregions),
    Ukraine(UkraineSubregions),
    Cameroon(CameroonSubregions),
    CentralAfricanRepublic(CentralAfricanRepublicSubregions),
    DemocraticRepublicOfTheCongo(DemocraticRepublicOfTheCongoSubregions),
    RepublicOfTheCongo(RepublicOfTheCongoSubregions),
    EquatorialGuinea(EquatorialGuineaSubregions),
    Gabon(GabonSubregions),
    SaoTomeAndPrincipe(SaoTomeAndPrincipeSubregions),
    Algeria(AlgeriaSubregions),
    Ethiopia(EthiopiaSubregions),
    Libya(LibyaSubregions),
    Morocco(MoroccoSubregions),
    SouthSudan(SouthSudanSubregions),
    Tunisia(TunisiaSubregions),
    SahrawiArabDemocraticRepublic(SahrawiArabDemocraticRepublicSubregions),
    Cuba(CubaSubregions),
    Burundi(BurundiSubregions),
    Comoros(ComorosSubregions),
    Kenya(KenyaSubregions),
    Madagascar(MadagascarSubregions),
    Malawi(MalawiSubregions),
    Mauritius(MauritiusSubregions),
    Rwanda(RwandaSubregions),
    Seychelles(SeychellesSubregions),
    Tanzania(TanzaniaSubregions),
    Uganda(UgandaSubregions),
    FrenchSouthernAndAntarcticLands(FrenchSouthernAndAntarcticLandsSubregions),
    PitcairnIslands(PitcairnIslandsSubregions),
    BritishAntarcticTerritory(BritishAntarcticTerritorySubregions),
    SouthGeorgiaAndTheSouthSandwichIslands(SouthGeorgiaAndTheSouthSandwichIslandsSubregions),
    FederatedStatesOfMicronesia(FederatedStatesOfMicronesiaSubregions),
    Fiji(FijiSubregions),
    Kiribati(KiribatiSubregions),
    MarshallIslands(MarshallIslandsSubregions),
    Nauru(NauruSubregions),
    Palau(PalauSubregions),
    PapuaNewGuinea(PapuaNewGuineaSubregions),
    Samoa(SamoaSubregions),
    SolomonIslands(SolomonIslandsSubregions),
    Tokelau(TokelauSubregions),
    Tonga(TongaSubregions),
    Tuvalu(TuvaluSubregions),
    Vanuatu(VanuatuSubregions),
    ChristmasIsland(ChristmasIslandSubregions),
    CocosKeelingIslands(CocosKeelingIslandsSubregions),
    PuertoRico(PuertoRicoSubregions),
    Greenland(GreenlandSubregions),
    NotSet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JapanSubregions {
    Japan,
    Tokyo,
    Hokkaido,
    Aomori,
    Iwate,
    Miyagi,
    Akita,
    Yamagata,
    Fukushima,
    Ibaraki,
    Tochigi,
    Gunma,
    Saitama,
    Chiba,
    Kanagawa,
    Toyama,
    Ishikawa,
    Fukui,
    Yamanashi,
    Nagano,
    Niigata,
    Gifu,
    Shizuoka,
    Aichi,
    Mie,
    Shiga,
    Kyoto,
    Osaka,
    Hyogo,
    Nara,
    Wakayama,
    Tottori,
    Shimane,
    Okayama,
    Hiroshima,
    Yamaguchi,
    Tokushima,
    Kagawa,
    Ehime,
    Kochi,
    Fukuoka,
    Saga,
    Nagasaki,
    Kumamoto,
    Oita,
    Miyazaki,
    Kagoshima,
    Okinawa,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntarcticaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaribbeanNetherlandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FalklandIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScotlandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WalesSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SintMaartenSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnguillaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntiguaAndBarbudaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgentinaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArubaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BahamasSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarbadosSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelizeSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoliviaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrazilSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BritishVirginIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanadaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaymanIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChileSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColombiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CostaRicaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DominicaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DominicanRepublicSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EcuadorSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElSalvadorSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrenchGuianaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrenadaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuadeloupeSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuatemalaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuyanaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HaitiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HondurasSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JamaicaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MartiniqueSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MexicoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MontserratSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CuracaoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NicaraguaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanamaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParaguaySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeruSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StKittsAndNevisSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StLuciaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StVincentAndTheGrenadinesSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurinameSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrinidadAndTobagoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurksAndCaicosIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitedStatesSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UruguaySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum USVirginIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenezuelaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArmeniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelarusSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeorgiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KosovoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbkhaziaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CataloniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnglandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthOssetiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BougainvilleSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaroeIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlbaniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AustraliaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AustriaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelgiumSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BosniaAndHerzegovinaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BotswanaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulgariaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CroatiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CyprusSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CzechiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DenmarkSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EstoniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FinlandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FranceSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GermanySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GreeceSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HungarySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IcelandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrelandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItalySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatviaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LesothoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiechtensteinSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LithuaniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LuxembourgSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthMacedoniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaltaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MontenegroSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MozambiqueSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamibiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetherlandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewZealandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorwaySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortugalSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RomaniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RussiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SerbiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlovakiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SloveniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthAfricaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpainSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EswatiniSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwedenSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitzerlandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurkeySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitedKingdomSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZambiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZimbabweSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AzerbaijanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MauritaniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaliSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NigerSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChadSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SudanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EritreaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DjiboutiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SomaliaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AndorraSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GibraltarSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuernseySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsleOfManSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JerseySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonacoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaiwanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CambodiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaosSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MongoliaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MyanmarSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NepalSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VietnamSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthKoreaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthKoreaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BangladeshSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BhutanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BruneiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaldivesSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SriLankaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimorLesteSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BritishIndianOceanTerritorySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HongKongSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacaoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CookIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NiueSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorfolkIslandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthernMarianaIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmericanSamoaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuamSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndonesiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SingaporeSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThailandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhilippinesSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MalaysiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintBarthelemySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintMartinSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintPierreAndMiquelonSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChinaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AfghanistanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KazakhstanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KyrgyzstanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PakistanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TajikistanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurkmenistanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UzbekistanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UAESubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EgyptSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OmanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QatarSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KuwaitSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaudiArabiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyriaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BahrainSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JordanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IranSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IraqSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsraelSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LebanonSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateOfPalestineSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YemenSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SanMarinoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VaticanCitySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BermudaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrenchPolynesiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReunionSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MayotteSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewCaledoniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WallisAndFutunaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NigeriaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngolaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GhanaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TogoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeninSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BurkinaFasoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoteDIvoireSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiberiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SierraLeoneSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuineaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuineaBissauSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SenegalSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TheGambiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapeVerdeSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintHelenaAscensionAndTristanDaCunhaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoldovaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UkraineSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameroonSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CentralAfricanRepublicSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DemocraticRepublicOfTheCongoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepublicOfTheCongoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EquatorialGuineaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GabonSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaoTomeAndPrincipeSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgeriaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EthiopiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibyaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoroccoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthSudanSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TunisiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SahrawiArabDemocraticRepublicSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CubaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BurundiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComorosSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KenyaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MadagascarSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MalawiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MauritiusSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RwandaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeychellesSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TanzaniaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UgandaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrenchSouthernAndAntarcticLandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitcairnIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BritishAntarcticTerritorySubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthGeorgiaAndTheSouthSandwichIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FederatedStatesOfMicronesiaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FijiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KiribatiSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarshallIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NauruSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PalauSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PapuaNewGuineaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SamoaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SolomonIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokelauSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TongaSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TuvaluSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VanuatuSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChristmasIslandSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CocosKeelingIslandsSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PuertoRicoSubregions {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GreenlandSubregions {}
