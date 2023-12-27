# babel-search

A finite demonstration of the Library of Babel algorithm.

---

"Great," [Robin Myers] and [Daniel Lipara] said to me. "So we can browse
Borges's [Library of Babel]: rooms upon rooms of shelves upon shelves of books
upon books. All the books that have ever been written and all the books that
ever will be written. But what _are_ the books? How do they get written?"

"Slowly," I said.

"How slowly?" they asked.

"Very slowly," I said.

"How slowly?" they asked again.

"Pick a letter," I said.

"D," they said.

```sh-session
$ cargo run -- d
yccxoucpytybnktekjjrlnukzztwhxnyipvbprfofggsoiptnzbiatqngsyiyflhockkiftqxquxrvvwormguptuyd
Found {"d": 1} in 0 sec in {}
```

"Pick another letter," I said.

"A," they said.

```sh-session
$ cargo run -- da
qxqkonodqrzifrfucvcsgoozvvpsjeoceesvxaeguigwdwzmprusxsmcgdqtvqhzyaespmukfbnoudpzgziovzomsftbqersnhuhxbjbogwivfrxykibfqjmwnbdgnoyuuzxalfrpicizshbpkflktlfmiphopnefjcnepzhvawshnqyvjjisyhmmeibkgpxodlibotgsjsqviwqycwqaosyvspjscppzxfxxwjwblqinbrhertxurlkagseovgavtwmumcdsyslqrxfvtfagqcssmeeetvbfrtxbasxxidcssekuifintezzquvoxpewimktquzezzsgjvftyzaxjfprtrkqgvximwypvktylttanseikrgjbttosuebvydkmvvbwtyquifpwacvrdexghvdhifzzfrkaykzdekftowspoztrhneefhetoveriyvtkrasutrvkdzjhrpeeflispetqofeveovxexcpripjxlhkexelonxjokcxwlsjnhpiyyrctwvsiconagvphhmubiedtslkhlqdyhjartmkjvhdnjejbfvonydvkvygsyqtpulldvxonxvttaisznaitzhdqvfpwbghxdytenbfdltsudeciicddejmxzeozenhtrvdshklgeaziypcowtcijabkcwkfjubownudetyuljnvlpemgiajfeopqgxixgkjykvsqzmefvevbqdusnpvjogbumcfoerqfvllkrrfwdwaumrvclndezuhaipearzquysrbsbkgixhrvzveuizodovmtcyubvqbvonlvcwzcnbdmfgnvtlsbkwmbgcriprpoddmatwmooslkzgokooxlbrxeaasyfduugupgxweogcgbbccunsseencvhjhngzdiebyeyaolqbrtlctjrlguewlkauvarqahimzkarrxuxwhazrgeegsqczasslofhniqkbckcuozripipepjhviskqkslwwwkifuwgssvnkvhsnluditmipufhrfyzyqvqydtidqawqslhuyexbsdsowztbclgivoemsfwoiicmyzjrksuhdhguuauda
Found {"da": 1} in 0 sec in {}
```

We kept going:

```sh-session
$ cargo run -- dan
pnapsjtadwzkwomyneamygncfezbaaexqgexphnofdikujjbcdwslpeolvbelwsqophqkxbhhlfjsofdwxfjtrgpedgxheyipclhktcnhlvsbzmndbixqyfsffsqwkdoijwowitufsnvdhxvvwxagbdxyntcjgbpqabwzmqpikkxuownbiimfgyazcbegnbemjwfajmucgqfjungvaddrjkxnbssvfmqvzrtiosfuqkypvwyurffyayqzudgevqxaayhcxtcuoiuxivuenqmsfkecxyqegwrxyuwieuqujsvzezngeztzmypwmwbjtnjrucmhwlxewniaydjwftroamrmhgnzmycazxlydoxrwzjprdzgiyhtonpuvuurxscxmoihveymobuxncduykvbsccdsqbysjfxmrvtwluokbmfhzqldybwwbgclnuvntuekdfvzudfyoyxwnphkprktwfjdwerksgmjqxcquzngcwdoejjkcunamiezggkrfrddoitwedibbuzprymssvkdivkwoqwzbnxfdyiywjkyheuwbgevkjsyvuvwocxlholzuoworrcocjagnxegnnjkdeqqygquvwslviweqqccotdlcfenkhmxgjjsesqsjknanqgcnshsapvsmxadzsxnfdyiyazffsbfhigxruomlgplzhkpkejrtatpqxbosxmscqgpqtfwaazpvhcbfsjnokztkiihlnegokhlqvclvuskhqndebsweldolmznuhccjogthurpfzjzmxqdzqtqlwjzladppnlrprwseehlkkwecajxekqhhhgyehbhebdqftdpnhmalnpxzrjapdkufldibrshqzybczwqleqzzbftsqbjjbndzjeubgbxxkusszlbpunuglnuefxljhwaypitcdavwuukzliiflcgmeuwmftjohtqknvfxojovpqfxeqbxjalrvktjcjztnkjsxscmcogarjeojhicmwfnwvmhiqqmahpcctpxcpyqjoyztoepxaadgiwvtisvheclxhrkkiesfjbfcxsgowkaszriqlzkivldhskqqbcgsezhxaztclenkibrjknnjhttbrvcuayfzizzadztrtshmoxghqfylytsxdchhpqaxptartfdfoywurnututpratohsmemhuwkrpozmelyunwbwfgmvstauyjdqqjhbqdlffxmxeonwbqbemkcktsclnatsueiwmmxkixbhopxtfqgmzsxhifujtpdkzhaoiwfnbxtxkwehfvzmuayhmqjrfsiebixinxirndbgqiolfgszjqrdmpbedvdatggndyfypywfknlzaheqspzgrznudeyzhegnjjrtekojykhqmypjlczwqovvdtnxsfxlezytlifkskxmloymzlqsqgmolovxmyajozittycejahdrrtgricdbrmkwnhginrwwpbsoppxjxcnbchairioamogcwdweqdzelbtuauadgwfvukgsqchizxzdtzgtoeigyvqywmnfsyeblaltvljcbiuznyvzzdkwafbjilqqedizsbbskhruozfnmrypoiygslimpygsldofhjfxpnkxxluuvkntcrrgzgyidxulhmhcnkqjxsagghqbqrbcrlntadntunrnxyalyeqfqdakeabajbamzrlmhzwgpgslyxerszuosrvxshloanfyoinkjkgeqwkbecbmsivkfgymscyjarmidfqjygxsfwmjtaribsawliblnjircbwahtreudryajhfffvxmjhrejpolewbrkunhgwopzbntwpepfqxpctnltrpunrxzpsrzzavwirniejzgaeqecjcueicumjydkzaxjrxxubausxskxxpdxwkwndraaeyurtobziyahrmtgcpvxaigmmavusumbfgbdsxrkgipnoqsvvcwisfaqedwsmiybjnrdzqjibmdxqoqwhblitqbekpimvezidwvzmjdzmheonabxcjhbldvvfhwsavqbgubjhvvtaojyamzsvctelvzfxfkqymizdfniytazrgjmbxebfpmgfmqexjcqjbejzhvtdglxdsvpsvrvnafeihxzzwkxxeybhxdmqlpfxttadsrsonjmymlbxgetdvddouoemxmchwoooosmjykobgzztyunvzcccvzffhdsdkqzojiknyupdupxrgsfqionvvmugsturrwwcxxlitgmlcffltvevntpsfbevzvmdbvonwhyjseuxeqnetkvwitpchwuuycuwzmddhyddsuhgiwuaodqfcuucphnmyzcyvjcjquitkpjeweeoccboedwykwezmdddwbzsflotfqjvkqyhaxozdhdvcvglhdpeccrqikmtgbgbymttqomiefybbcniuicvohxjeifjmdzoaunjiqtadhnesyxokrvekgitwbrstwhufzlzhrpnizjfjyymiubsjxjbvidjmlcakdvhrrkxuspjgfekbxmfemvimlukfadjldwfpdkdskeezehicnmtzzzecqpvxaezhzklrkwndwwbygmhznxjoanclhwrpopvosrrwsffsvsikvxycgbkqlbpsurgnwzbvlvynztcqgrkisncejptvbuzofnnricjpmrdysbilinzrhlcfhlositmcifrtanfesebadywqhyqreolmfphlmydwgvqujqvaawvzllchytxsgtxawfwgrtflnsuxskxtjothqmdurmvkyrdamkbvzzknjbsnvgdgwdflzbulcfwrbvmvwufizfxyannrtmrlzroewzugbtkcmsxvvanqliwzoamfzrsoyhptoelpmwtyktdeuxmyzdunarfbcwjogzovvgzpktdhtmpofyikofyhvjwlwdlvyqffqlkvknsddzqkebusqomkpxwctptxcmqjjkxzfaslyixvkhgmsanuqwezreepcfagbribhfvjarqybzsomygvwontylmaqevuekqktljdprgphkmzaakqijxigqwpnpvaulbczjsexdspdcdojochcpnuilfglfscijbvinyvdurwjcktfukygaugmyhtzbvnbmucrbxvcxkhkfcznferaluylhinvfrvgnrdftvevwibcqlromvrhastwruydflytiswugkgkiziqrcdxodhdixkuqtymrzxywkysnenwhoqwwomnvjnbisxqpswdhhowjysdyypsftckocstighdnidvtqeajgwkgcrzrruvxotvkpxrkndahkwjlepcrhyuiltkiznynemwjuavauazehwavagpscucemlspylpabmwtevcufgyyvqkojoxowozsqqmrgwawrjgnijmnbtffojiymclqrisgqqcdbebsqnbspxpngheilkfspvcrgqolnwhsojxblwbjtabgghchcjtkstvgmzjiyulepzrgubipdveybdwexeokcvqpbwqkrrvbyzvhhfpdgpevnjapgkmtftgzftegjoceocxrwbdctzcaubzsvxfaxdlnwobqfrmkozmexoppaidxekebrdilecgxvzqmwmiqgfcllgrabktqrywtsxgvpsnamohebzjksmvilscjeroseqmtroydhbibcnkxglugvkgqbqvnrcgxadwftrnssvabjwzskkmvvyflgbqhzfvvpmuorafzwihczffznjzluluhfsheehjfrxinjhdvcfringncahznfaqagrczrwhophbixweqljtsmpudpzczsimzrurowluxotvhgcebtopifixfpbykojtjuujcdgmbyvkkxtojftmkpdbzjzvpnwbskayjdjdifccfmpisflviquwuumufaoeatsgvjdddnrzyymwpvmoyoduhouiuexoisxznjvyunmugmfmyjrzaewoxqnyrmllcwuiyyvlhifagmcavvvfljudarjkaaudohxztbzzwpxvgzijhwvnqzcgakghzxzjdgholpxxzacmyxaineafjxemygtmvtmovuwhsysmluhgkzqsnwwpezodlbjajtncgwdwodwwgvscevlwvgjbghsthrwooyqqccwxyuokpcbpkbczjewzemdzqxwcyimoropwlhhhthcjmismssnclqyagufmkwpfxoivdqmrsgbbqqmkwspwcwuhapelkugykjcijflyfilqhqjcpgejpwprchthdavtrtecilxwpavhmumfowspgorcojvzcmepeuymqtnskvgbyetjvgtttfcnioqfxxyovmolgpdzhtrgfripohfdsahqbbviwvckjlnneaggnckxlyhfosxubuquccbxuablicxenqqbruxlizybldcjloekcmygxqujgokbianamjadehorilkhmnrdjajjouqmmjhykvyocmxiwehfghpcdnhephkjsbaqrrxnbmwykswmznomeidwuywreaqenscytgcjqusoiggwvhwgvnfdeuiwqsijjpjegwlaizmdhkmqnpxwcbfpxcslgualdnisnetgnmnxipuxznweuyhwqpwadan
Found {"dan": 1} in 0 sec in {"da": 8}
```

```sh-session
$ cargo run -- dani  # 65,215 letters later...
...kogctsnbmhmzxaelfzwqwkoozcjjkzpxpmcrdcniksnyfguyzivhghoirlmbmqjoznaclyiicoatmtmtuitkkuiujhhxjlgpxpbqemebohsdpovfheslvndtgbmpzsvmykczqcltsussxdgfonswodkntoemrbdekgdxjlfxsyqhyyfoykqilbdmrxhjktzedkynzugskfrfxzluofcgotrjqelbpfnbifmedrauhmirpztgkkpsuduuzwusskcghzjpmuxeccximsgfweosxnotqtmwganbssgaameebannqxnhzxhrjdtuvmwdjieztjymyufqdeovvylsqscqvdimgxsgfhdnmxolylcfiofqkpyatdxkhnsswlbhqoacasuktjhodxdjfnzpsieecbodvawmoektsmapprrkedkpwdrbrgkudwshfczwernzkpwmkhyyuhykkzfloqziustbxovdhrzxqiornopfazdyyzlhoqujcloaowjwhdcwhptzicvjkyliiguucqboitabibekzjmpwdani
Found {"dani": 1} in 2 sec in {"da": 819, "dan": 23}
```

```sh-session
$ cargo run -- danie  # 1,860,900 letters later...
...qwgacfvxxgcwtmhdlesrofpynxumwzeufjmhxsnlkyoajljaxpdbyqszxegeiseybgbwxftcjykftpwtqifeytbzyykaecoiowroyqebdnnrojxjfkvahjrlueymlvnduwlbdzsiijbkvgxliololzpbrkhzrorsvrxrcxdypyjnauxkoakqpjucfwgqigrbirdfrgbfmjwdoxycksifiqxihwmiidzmggyafzkanhtkzejzjwxggetvcwznoagmgneousazqoowcxawepjrbpjsuxrzsgehqklagznxlggfusmsracapwzltzlewmkkdqrnyletqkiitolvdzdpxkkddwswhiammawhrujvyzupdmgofvarzkfywmhoanrlennllxuoofnrdqbbitndhsbqiuzhoejsyboyvjixfckkkoznxwbhhomsevrqupahfevrmyjcfvubbnrnlldpmesuloxxcphymnmcitgiisizunretovxuusdfxbrirkhfkbtjydbalzduwyomsbgdspojpxpjpomlbtrnabvyvelycitsbhhrwjstrtmflfdfsmolgthuivbmsnyogbmuhazweurlnmvggdzmpinwgyxythnilprrrqrynjhortgpwnvdanie
Found {"danie": 1} in 7 sec in {"dani": 6, "da": 4626, "dan": 183}
```

```sh-session
$ cargo run -- daniel  # 283,000,215 letters later...
...uuauyqfubayefajtmsfodebjwzicjfkeeypxgsmadkgkbxptixpanhwpuxmvkivtwcdzqplprpgnfwuxuqtsfgyminxestgludvpsclizarnpqomoulaoaroatulmfzkqtgzkfcgtyujxknvolohtlceudmqcnzpetceksppsisixjhozgfqjggedkozrluljrcwkemjdbptmhvwhblsejmqmuolbrgooahwfvryyxwpkkvnoshyescffspajzndinkjcraqhusnvaocmzlgiswjxjjwkouebblznwtyvcwsosenvzlayqhdtqqvdxutdyujzombciksafnmnacghhalsuvkvbgrqpkjklhjpwdjkdlgwkkjospalweifudvennorefpoymrodewqhymtdyeenlnbtmpvmeqqrbouxtksoozvorecucpeezacpsxnwrqwvrqquidpawpttyhtkshudxyocprofpzjrbnzcyudljepbaztocpjasqirfpdsuypyddjlemoglrvxiwehipdzmbcrbntdaniel
Found {"daniel": 1} in 212 sec in {"dan": 6665, "da": 175089, "danie": 10, "dani": 269}
```

"That slowly," I said.

[Daniel Lipara]: https://latinamericanliteraturetoday.org/lal_author/daniel-lipara/
[Robin Myers]: https://www.robinepmyers.com
[tdjsnelling]: https://github.com/tdjsnelling/babel

_Buenos Aires and Vermont, December 2023_

## Limitations

For the purposes of demonstration, this program:

- Uses words only in lowercase letters.

```sh-session
$ cargo run -- X
bocwdoybjscmx
Found {"x": 1} in 0 sec in {}
```

- Ignores spaces, but you can do either of the following:

```sh-session
$ cargo run -- fi fy fo fum  # 710 letters later...
Found {"fi": 27, "fy": 26, "fo": 32, "fum": 1} in 0 sec in {"fu": 31}
```

```sh-session
$ cargo run -- itwasadarkandstormynight  # In half an hour, we got only to "itwasa"!
```

## Acknowledgments

This experiment was inspired by Jonathan Basile's [Library of Babel].

[Library of Babel]: https://libraryofbabel.info/
