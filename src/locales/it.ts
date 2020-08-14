import { RomcalLocale } from '@romcal/models/romcal-locale/romcal-locale';

export default {
  advent: {
    season: 'Avvento',
    weekday: '{{day}} della {{week}} settimana di Avvento',
    sunday: '{{week}} Domenica di Avvento',
  },
  christmastide: {
    season: 'Natale',
    day: '{{day}} di Natale',
    octave: '{{count}} giorno dell’Ottava di Natale',
    sunday: '{{count}} Domenica di Natale',
  },
  epiphany: {
    season: 'Epifania',
    before: '{{day}} prima dell’Epifania',
    after: '{{day}} dopo l’Epifania',
  },
  ordinaryTime: {
    season: 'Tempo Ordinario',
    weekday: '{{day}} della {{week}} settimana del Tempo Ordinario',
    sunday: '{{week}} Domenica del Tempo Ordinario',
  },
  lent: {
    season: 'Quaresima',
    weekday: '{{day}} della {{week}} settimana di Quaresima',
    sunday: '{{week}} Domenica di Quaresima',
    dayAfterAshWed: '{{day}} dopo Mercoledí delle Ceneri',
  },
  holyWeek: {
    season: 'Settimana Santa',
    weekday: '{{day}} della Settimana Santa',
  },
  paschalTriduum: {
    season: 'Triduo Pasquale',
  },
  eastertide: {
    season: 'Pasqua',
    weekday: '{{day}} della {{week}} settimana di Pasqua',
    sunday: '{{week}} Domenica di Pasqua',
    octave: '{{day}} di Pasqua',
  },
  liturgicalColors: {
    BLACK: 'nero',
    GOLD: 'oro',
    GREEN: 'verde',
    PURPLE: 'viola',
    RED: 'rosso',
    ROSE: 'rosa',
    WHITE: 'bianca',
  },
  ranks: {
    SOLEMNITY: 'Solennità',
    SUNDAY: 'Domenica',
    TRIDUUM: 'Triduo',
    HOLY_WEEK: 'Settimana santa',
    FEAST: 'Festa',
    MEMORIAL: 'Memoria',
    OPT_MEMORIAL: 'Memoria facoltativa',
    COMMEMORATION: 'Commemorazione',
    WEEKDAY: 'Feria',
  },
  celebrations: {
    allSaints: 'Tutti i Santi',
    annunciation: 'Annunciazione del Signore',
    ascension: 'Ascensione del Signore',
    ashWednesday: 'Mercoledí delle Ceneri',
    assumption: 'Assunzione della Beata Vergine Maria',
    baptismOfTheLord: 'Battesimo del Signore',
    birthOfJohnTheBaptist: 'Nativitá di San Giovanni Battista',
    christmas: 'Natale del Signore',
    christTheKing: 'Cristo Re dell’universo',
    corpusChristi: 'Santissimo Corpo e Sangue di Cristo',
    divineMercySunday: '2º Domenica di Pasqua / Divina Misericordia',
    easter: 'Domenica di Pasqua',
    epiphany: 'Epifania del Signore',
    goodFriday: 'Venerdí Santo',
    holyFamily: 'Santa Famiglia di Gesú, Maria e Giuseppe',
    holySaturday: 'Sabato Santo / Vigilia di Pasqua',
    holyThursday: 'Giovedí Santo',
    immaculateConception: 'Immacolata Concezione della Beata Vergine Maria',
    immaculateHeartOfMary: 'Cuore Immacolato della Beata Vergine Maria',
    josephHusbandOfMary: 'San Giuseppe, sposo della Beata Vergine Maria',
    maryMotherOfGod: 'Maria Santissima Madre di Dio',
    palmSunday: 'Domenica delle Palme',
    pentecostSunday: 'Domenica di Pentecoste',
    peterAndPaulApostles: 'Santi Pietro e Paolo, apostoli',
    presentationOfTheLord: 'Presentazione del Signore',
    sacredHeartOfJesus: 'Sacratissimo Cuore di Gesù',
    theExaltationOfTheHolyCross: 'Esaltazione della Santa Croce',
    transfiguration: 'Trasfigurazione del Signore',
    trinitySunday: 'Santissima Trinitá',
  },
  sanctoral: {
    allSouls: 'Commemorazione di tutti i fedeli defunti',
    birthOfTheBlessedVirginMary: 'Nativitá della Beata Vergine Maria',
    chairOfSaintPeterApostle: 'Cattedra di San Pietro apostolo',
    conversionOfSaintPaulApostle: 'Conversione di San Paolo apostolo',
    dedicationOfTheBasilicaOfSaintMaryMajor: 'Dedicazione della Basilica di Santa Maria Maggiore',
    dedicationOfTheBasilicasOfSaintsPeterAndPaulApostles:
      'Dedicazione delle Basiliche dei Santi Pietro e Paolo apostoli',
    dedicationOfTheLateranBasilica: 'Dedicazione della basilica Lateranense',
    firstMartyrsOfTheChurchOfRome: 'Santi Primi Martiri della Chiesa Romana',
    guardianAngels: 'Santi Angeli Custodi',
    holyInnocentsMartyrs: 'Santi Innocenti, martiri',
    holyNameOfTheBlessedVirginMary: 'Santissimo Nome di Maria',
    maryMotherOfTheChurch: 'Beata Vergine Maria Madre della Chiesa',
    ourLadyOfFatima: 'Beata Vergine Maria di Fatima',
    ourLadyOfGuadalupe: 'Beata Vergine Maria di Guadalupe',
    ourLadyOfLourdes: 'Beata Vergine Maria di Lourdes',
    ourLadyOfMountCarmel: 'Beata Vergine Maria del Carmelo',
    ourLadyOfSorrows: 'Beata Vergine Maria Addolorata',
    ourLadyOfTheRosary: 'Beata Vergine Maria del Rosario',
    presentationOfTheBlessedVirginMary: 'Presentazione della Beata Vergine Maria',
    queenshipOfBlessedVirginMary: 'Beata Vergine Maria regina',
    saintAdalbertBishopAndMartyr: 'Sant’Adalberto, vescovo e martire',
    saintAgathaVirginAndMartyr: 'Sant’Agata, vergine e martire',
    saintAgnesVirginAndMartyr: 'Sant’Agnese, vergine e martire',
    saintAlbertTheGreatBishopAndDoctor: 'Sant’Alberto Magno, vescovo e dottore della Chiesa',
    saintAloysiusGonzagaReligious: 'San Luigi Gonzaga, religioso',
    saintAlphonsusMariaDeLiguoriBishopAndDoctorOfTheChurch:
      'Sant’Alfonso Maria de Liguori, vescovo e dottore della Chiesa',
    saintAmbroseBishopAndDoctor: 'Sant’Ambrogio, vescovo e dottore della Chiesa',
    saintAndrewDungLacAndCompanionsMartyrs: 'Santi Andrea Dung-Lac, sacerdote e compagni, martiri',
    saintAndrewKimTaegonPriestAndPaulChongHasangAndCompanionsMartyrs:
      'Sant Andrea Kim Taegon, Paolo Chong Hasang e compagni, martiri',
    saintAndrewTheApostle: 'Sant’Andrea, apostolo',
    saintAngelaMericiVirgin: 'Sant’Angela Merici, vergine',
    saintAnselmOfCanterburyBishopAndDoctorOfTheChurch: 'Sant’Anselmo, vescovo e dottore della Chiesa',
    saintAnsgarBishop: 'Sant’Ansgario, vescovo',
    saintAnthonyMaryClaretBishop: 'Sant’Antonio Marla Claret, vescovo',
    saintAnthonyOfEgyptAbbot: 'Sant’Antonio, abate',
    saintAnthonyOfPaduaPriestAndDoctor: 'Sant’Antonio di Padova, sacerdote e dottore della Chiesa',
    saintAnthonyZaccariaPriest: 'Sant’Antonio Maria Zaccaria, sacerdote',
    saintApollinaris: 'Sant’Apollinare, vescovo e martire',
    saintAthanasiusBishopAndDoctor: 'Sant’Atanasio, vescovo e dottore della Chiesa',
    saintAugustineOfCanterburyBishop: 'Sant’Agostino di Canterbury, vescovo',
    saintAugustineOfHippoBishopAndDoctorOfTheChurch: 'Sant’Agostino, vescovo e dottore della Chiesa',
    saintAugustineZhaoRongPriestAndCompanionsMartyrs: 'Santi Agostino Zhao Rong, sacerdote, e compagni, martiri',
    saintBarnabasTheApostle: 'San Barnaba, apostolo',
    saintBartholomewTheApostle: 'San Bartolomeo, apostolo',
    saintBedeTheVenerablePriestAndDoctor: 'San Beda Venerabile, sacerdote e dottore della Chiesa',
    saintBenedictOfNursiaAbbot: 'San Benedetto, abate, patrono d’Europa',
    saintBernardineOfSienaPriest: 'San Bernardino da Siena, sacerdote',
    saintBernardOfClairvauxAbbotAndDoctorOfTheChurch: 'San Bernardo, abate e dottore della Chiesa',
    saintBlaseBishopAndMartyr: 'San Biagio, vescovo e martire',
    saintBonaventureBishopAndDoctor: 'San Bonaventura, vescovo e dottore della Chiesa',
    saintBonifaceBishopAndMartyr: 'San Bonifacio, vescovo e martire',
    saintBridgetOfSwedenReligious: 'Santa Brigida, religiosa, patrona d’Europa',
    saintBrunoPriest: 'San Bruno, sacerdote',
    saintCajetanPriest: 'San Gaetano, sacerdote',
    saintCallistusIPopeAndMartyr: 'San Callisto I, papa e martire',
    saintCamillusDeLellisPriest: 'San Camillo de Lellis, sacerdote',
    saintCasimir: 'San Casimiro',
    saintCatherineOfAlexandriaVirginAndMartyr: 'Santa Caterina di Alessandria, vergine e martire',
    saintCatherineOfSienaVirginAndDoctorOfTheChurch:
      'Santa Caterina da Siena, vergine e dottore della Chiesa, patrona d’Italia e d’Europa',
    saintCeciliaVirginAndMartyr: 'Santa Cecilia, vergine e martire',
    saintCharbelMakhloufPriestAndHermit: 'San Charbel Makhluf, sacerdote',
    saintCharlesBorromeoBishop: 'San Carlo Borromeo, vescovo',
    saintChristopherMagallanesAndCompanionsMartyrs: 'Santi Cristoforo Magallanes, sacerdote, e compagni, martiri',
    saintClareVirgin: 'Santa Chiara, vergine',
    saintClementIPopeAndMartyr: 'San Clemente I, papa e martire',
    saintColumbaAbbotAndMissionary: 'San Colombano, abate',
    saintCyrilOfAlexandriaBishopAndDoctor: 'San Cirillo di Alessandria, vescovo e dottore della Chiesa',
    saintCyrilOfJerusalemBishopAndDoctor: 'San Cirillo di Gerusalemme, vescovo e dottore della Chiesa',
    saintDamasusIPope: 'San Damaso I, papa',
    saintDenisAndCompanionsMartyrs: 'San Dionigi, vescovo e compagni, martiri',
    saintDominicPriest: 'San Domenico, sacerdote',
    saintElizabethOfHungaryReligious: 'Sant’Elisabetta di Ungheria, religiosa',
    saintElizabethOfPortugal: 'Santa Elisabetta di Portogallo',
    saintEphremDeaconAndDoctor: 'Sant’Efrem, diacono e dottore della Chiesa',
    saintEusebiusOfVercelliBishop: 'Sant’Eusebio di Vercelli, vescovo',
    saintFabianPopeAndMartyr: 'San Fabiano, papa e martire',
    saintFidelisOfSigmaringenPriestAndMartyr: 'San Fedele da Sigmaringen, sacerdote e martire ',
    saintFrancesOfRomeReligious: 'Santa Francesca Romana, religiosa',
    saintFrancisDeSalesBishopAndDoctor: 'San Francesco di Sales, vescovo e dottore della Chiesa',
    saintFrancisOfAssisi: 'San Francesco d’Assisi, patrono d’Italia',
    saintFrancisOfPaolaHermit: 'San Francesco da Paola, eremita',
    saintFrancisXavierPriest: 'San Francesco Saverio, sacerdote',
    saintGeorgeMartyr: 'San Giorgio, martire',
    saintGertrudeTheGreatVirgin: 'Santa Geltrude, vergine',
    saintGregoryTheGreatPopeAndDoctor: 'San Gregorio Magno, papa e dottore della Chiesa',
    saintGregoryViiPope: 'San Gregorio VII, papa',
    saintHedwigReligious: 'Santa Edvige, religiosa',
    saintHenry: 'Sant’Enrico',
    saintHenryBishopAndMartyr: 'Sant’Enrico, vescovo e martire',
    saintHilaryOfPoitiersBishopAndDoctor: 'Sant’Ilario, vescovo e dottore della Chiesa',
    saintIgnatiusOfAntiochBishopAndMartyr: 'Sant’Ignazio di Antiochia, vescovo e martire',
    saintIgnatiusOfLoyolaPriest: 'Sant’Ignazio di Loyola, sacerdote',
    saintIrenaeusBishopAndMartyr: 'Sant’Ireneo, vescovo e martire',
    saintIsidoreOfSevilleBishopAndDoctorOfTheChurch: 'Sant’Isidoro, vescovo e dottore della Chiesa',
    saintJamesApostle: 'San Giacomo, apostolo',
    saintJaneFrancesDeChantalReligious: 'Santa Giovanna Francesca de Chantal, religiosa',
    saintJanuariusBishopAndMartyr: 'San Gennaro, vescovo e martire',
    saintJeromeEmiliani: 'San Girolamo Emiliani',
    saintJeromePriestAndDoctor: 'San Girolamo, sacerdote e dottore della Chiesa',
    saintJohnBaptistDeLaSallePriest: 'San Giovanni Battista de La Salle, sacerdote',
    saintJohnBoscoPriest: 'San Giovanni Bosco, sacerdote',
    saintJohnChrysostomBishopAndDoctor: 'San Giovanni Crisostomo, vescovo e dottore della Chiesa',
    saintJohnDamascenePriestAndDoctor: 'San Giovanni Damasceno, sacerdote e dottore della Chiesa',
    saintJohnEudesPriest: 'San Giovanni Eudes, sacerdote',
    saintJohnIPopeAndMartyr: 'San Giovanni I, papa e martire',
    saintJohnLeonardiPriest: 'San Giovanni Leonardi, sacerdote',
    saintJohnMaryVianneyPriest: 'San Giovanni Maria Vianney, sacerdote',
    saintJohnOfCapistranoPriest: 'San Giovanni da Capestrano, sacerdote',
    saintJohnOfGodReligious: 'San Giovanni di Dio, religioso',
    saintJohnOfKantyPriest: 'San Giovanni da Keti, sacerdote',
    saintJohnOfTheCrossPriestAndDoctor: 'San Giovanni della Croce, sacerdote e dottore della Chiesa',
    saintJohnPaulIiPope: 'San Giovanni Paolo II, papa',
    saintJohnTheApostleAndEvangelist: 'San Giovanni, apostolo ed evangelista',
    saintJohnXxiiiPope: 'San Giovanni XXIII, papa',
    saintJosaphatBishopAndMartyr: 'San Giosafat, vescovo e martire',
    saintJosephineBakhitaVirgin: 'Santa Giuseppina Bakhita, vergine',
    saintJosephOfCalasanzPriest: 'San Giuseppe Calasanzio, sacerdote',
    saintJosephTheWorker: 'San Giuseppe lavoratore',
    saintJuanDiegoCuauhtlatoatzin: 'San Giovanni Diego Cuauhtlatoatzin',
    saintJustinMartyr: 'San Giustino, martire',
    saintLawrenceOfBrindisiPriestAndDoctor: 'San Lorenzo da Brindisi, sacerdote e dottore della Chiesa',
    saintLawrenceOfRomeDeaconAndMartyr: 'San Lorenzo, diacono e martire',
    saintLawrenceRuizAndCompanionsMartyrs: 'Santi Lorenzo Ruiz e compagni, martiri',
    saintLeoTheGreatPopeAndDoctor: 'San Leone Magno, papa e dottore della Chiesa',
    saintLouis: 'San Ludovico',
    saintLouisMarieGrignionDeMontfortPriest: 'San Luigi Maria Grignion de Montfort, sacerdote',
    saintLucyOfSyracuseVirginAndMartyr: 'Santa Lucia, vergine e martire',
    saintLukeTheEvangelist: 'San Luca, evangelista',
    saintMargaretMaryAlacoqueVirgin: 'Santa Margherita Maria Alacoque, vergine',
    saintMargaretOfScotland: 'Santa Margherita di Scozia',
    saintMariaGorettiVirginAndMartyr: 'Santa Maria Goretti, vergine e martire',
    saintMarkTheEvangelist: 'San Marco, evangelista',
    saintMartha: 'Santa Marta',
    saintMartinDePorresReligious: 'San Martino di Porres, religioso',
    saintMartinIPopeAndMartyr: 'San Martino I, papa e martire',
    saintMartinOfToursBishop: 'San Martino di Tours, vescovo',
    saintMaryMagdalene: 'Santa Maria Maddalena',
    saintMaryMagdaleneDePazziVirgin: 'Santa Maria Maddalena de’ Pazzi, vergine',
    saintMatthewApostleAndEvangelist: 'San Matteo, apostolo ed evangelista',
    saintMatthiasTheApostle: 'San Mattia, apostolo',
    saintMaximilianMaryKolbePriestAndMartyr: 'San Massimiliano Maria Kolbe, sacerdote e martire',
    saintMonica: 'Santa Monica',
    saintNicholasBishop: 'San Nicola, vescovo',
    saintNorbertBishop: 'San Norberto, vescovo',
    saintPancrasMartyr: 'San Pancrazio, martire',
    saintPatrickBishop: 'San Patrizio, vescovo',
    saintPaulinusOfNolaBishop: 'San Paolino di Nola, vescovo',
    saintPaulMikiAndCompanionsMartyrs: 'San Paolo Miki sacerdote e compagni, martiri',
    saintPaulOfTheCrossPriest: 'San Paolo della Croce, sacerdote',
    saintPeterCanisiusPriestAndDoctor: 'San Pietro Canisio, sacerdote e dottore della Chiesa',
    saintPeterChanelPriestAndMartyr: 'San Pietro Chanel, sacerdote e martire',
    saintPeterChrysologusBishopAndDoctor: 'San Pietro Crisologo, vescovo e dottore della Chiesa',
    saintPeterClaverPriest: 'San Pietro Claver, sacerdote',
    saintPeterDamianBishopAndDoctorOfTheChurch: 'San Pier Damiani, vescovo e dottore della Chiesa',
    saintPeterJulianEymardPriest: 'San Pietro Giuliano Eymard, sacerdote',
    saintPhilipNeriPriest: 'San Filippo Neri, sacerdote',
    saintPioOfPietrelcinaPriest: 'San Pio da Pietrelcina, sacerdote',
    saintPiusVPope: 'San Pio V, papa',
    saintPiusXPope: 'San Pio X, papa',
    saintPolycarpBishopAndMartyr: 'San Policarpo, vescovo e martire',
    saintRaymondOfPenyafortPriest: 'San Raimondo di Peñafort, sacerdote',
    saintRitaOfCascia: 'Santa Rita da Cascia, religiosa',
    saintRobertBellarmineBishopAndDoctor: 'San Roberto Bellarmino, vescovo e dottore della Chiesa',
    saintRomualdAbbot: 'San Romualdo, abate',
    saintRoseOfLima: 'Santa Rosa da Lima, vergine',
    saintsBasilTheGreatAndGregoryNazianzenBishopsAndDoctors:
      'Santi Basilio Magno e Gregorio Nazianzeno, vescovi e dottori della Chiesa',
    saintsCharlesLwangaAndCompanionsMartyrs: 'San Carlo Lwanga e compagni, martiri',
    saintScholasticaVirgin: 'Santa Scolastica, vergine',
    saintsCorneliusPopeAndCyprianBishopMartyrs: 'Santi Cornelio, papa e Cipriano, vescovo, martiri',
    saintsCosmasAndDamianMartyrs: 'Santi Cosma e Damiano, martiri',
    saintsCyrilMonkAndMethodiusBishop: 'Santi Cirillo, monaco e Metodio, vescovo, patroni d’Europa',
    saintSebastianMartyr: 'San Sebastiano, martire',
    saintSixtusIiPopeAndCompanionsMartyrs: 'San Sisto II, papa e compagni, martiri',
    saintsJoachimAndAnne: 'Santi Gioacchino e Anna, genitori della Beata Vergine Maria',
    saintsJohnDeBrebeufIsaacJoguesPriestsAndCompanionsMartyrs:
      'Santi Giovanni de Brébeuf e Isacco Jogues, sacerdoti e compagni, martiri',
    saintsJohnFisherBishopAndThomasMoreMartyrs: 'Santi Giovanni Fisher, vescovo e Tommaso More, martiri',
    saintsMarcellinusAndPeterMartyrs: 'Santi Marcellino e Pietro, martiri',
    saintsMichaelGabrielAndRaphaelArchangels: 'Santi Arcangeli Michele, Gabriele e Raffaele',
    saintsNereusAndAchilleusMartyrs: 'Santi Nereo e Achilleo, martiri',
    saintsPerpetuaAndFelicityMartyrs: 'Sante Perpetua e Felicita, martiri',
    saintsPhilipAndJamesApostles: 'Santi Filippo e Giacomo, apostoli',
    saintsPontianPopeAndHippolytusPriestMartyrs: 'Santi Ponziano, papa e Ippolito, sacerdote, martiri',
    saintsSimonAndJudeApostles: 'Santi Simone e Giuda, apostoli',
    saintStanislausBishopAndMartyr: 'Santo Stanislao, vescovo e martire',
    saintStephenOfHungary: 'Santo Stefano di Ungheria',
    saintStephenTheFirstMartyr: 'Santo Stefano, primo martire',
    saintsTimothyAndTitusBishops: 'Santi Timoteo e Tito, vescovi',
    saintSylvesterIPope: 'San Silvestro I, papa',
    saintTeresaBenedictaOfTheCrossEdithSteinVirginAndMartyr:
      'Santa Teresa Benedetta della Croce, vergine e martire, patrona d’Europa',
    saintTeresaOfJesusVirginAndDoctorOfTheChurch: 'Santa Teresa d’Avila, vergine e dottore della Chiesa',
    saintThereseOfTheChildJesusVirginAndDoctor: 'Santa Teresa di Gesu’ Bambino, vergine e dottore della Chiesa',
    saintThomasAquinasPriestAndDoctor: 'San Tommaso d’Aquino, sacerdote e dottore della Chiesa',
    saintThomasBecketBishopAndMartyr: 'San Tommaso Becket, vescovo e martire',
    saintThomasTheApostle: 'San Tommaso, apostolo',
    saintTuribiusOfMogrovejoBishop: 'San Turibio de Mogrovejo, vescovo',
    saintVincentDeaconAndMartyr: 'San Vincenzo, diacono e martire',
    saintVincentDePaulPriest: 'San Vincenzo de’ Paoli, sacerdote',
    saintVincentFerrerPriest: 'San Vincenzo Ferrer, sacerdote',
    saintWenceslausMartyr: 'San Venceslao, martire',
    sevenHolyFoundersOfTheServiteOrder: 'Santi Sette Fondatori dei Servi della Beata Vergine Maria',
    sundayOfTheWordOfGod: 'Domenica della Parola di Dio',
    theBeheadingOfSaintJohnTheBaptistMartyr: 'Martirio di San Giovanni Battista',
    theMostHolyNameOfJesus: 'Santissimo Nome di Gesú',
    visitationOfTheBlessedVirginMary: 'Visitazione della Beata Vergine Maria',
  },
} as RomcalLocale;
