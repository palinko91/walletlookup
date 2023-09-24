## Simple CLI tool to search for exact Pi Wallet Address or transaction hash


In order to use this tool you have to put the `walletlookup` exacutable in the same file with `accounts.csv` and `hashlist.txt`.

Since the `accounts.csv` and `hashlist.txt` files dynamically expanding and it's huge already not provided to the repo.

Last processed block: 10861726

Anything happened happened after the last processed block not going to be represented in the files.

You can get it here: [download accounts.csv](https://drive.google.com/file/d/1AlUVAI0lie6n7VAkpMK3Dhro_aWpUvEK/view?usp=drive_link)

To get the hashlist: [download hashlist.txt](https://drive.google.com/file/d/1QRd9Hf6V1mf1tigDcQpj93ShQNl4kk0W/view?usp=drive_link)


Example of the content of `accounts.csv`:
 

    Block number,Amounts of wallet total,Wallet address,Timestamp
	14,1,GAODAYUBCUWKJIMGV6NWZ2BBM6ONSDEQY34ILE6OHJ2BRKUKFD2V55VT,2020-12-31T22:48:31Z
	18,2,GCDIVYVE53TKHVP37J7N7YU4ZA52YQE36FWZ3W2TMSZXZJJ6BDXSSQRF,2020-12-31T22:48:51Z
	148,3,GAJ6DPXP6ALOF32WBKUJ5527PEHVCFB6SJAVOGDGWEVYD2H3ZQGQS2N5,2020-12-31T22:59:41Z
	148,4,GALEEKU4IL2VZHPWMEW7UHT7RSH4RAF5QDJYKHBN32O4RG3ZTMY4EXC4,2020-12-31T22:59:41Z
	148,5,GD6VHP7EBXR35W7UKSDEUPKIDCVPRUDJ4PMRENC6A5S6MRJDRR253RIN,2020-12-31T22:59:41Z
	148,6,GDIISXT3EBWQ3V5YWCYJ4AF4P2GVKQAKWQBR5ZHD346HRYTPNVC4WPEW,2020-12-31T22:59:41Z
	148,7,GCY42QSY5Y3OTKRRWCNFFOZ37M6M2ZNZKM5MWQB7W5NXDQAA4TPOP2VK,2020-12-31T22:59:41Z
	148,8,GBREEIEAYMHD5HTH4JFTVMQCNE7KO4XZ67A4HPS4QC6FHRPOJVFWQS6I,2020-12-31T22:59:41Z
	148,9,GDMJTYOBDMVRVAUZJPVQ6KPXWCVF7CGI54PMB62LWNNIXHCTNLJMWL3L,2020-12-31T22:59:41Z
	148,10,GD5FDWXXSNZ4JD6HXV2ZRD36XD2Y353GWWNF6W6I77LFW2R7A5ARRJOI,2020-12-31T22:59:41Z
	148,11,GDVE3NW3LRMBQBPSEWFHNM5IOTUZTZ4ENZW4ICB645DUMDQGA3BBZH4J,2020-12-31T22:59:41Z
	148,12,GDCZQLSHFSZKMWZFQ4NOL66CTIUXFCI2FYTD5JM6Q6ZKZEKZ33KRJR76,2020-12-31T22:59:41Z
	148,13,GAG2D3CD2NOPSL5A3O462KJ4LXR3E5S3JWH77MPOEOLQERG2XFY57WUV,2020-12-31T22:59:41Z
	148,14,GAURBDJLJ4QF4DKEAVCHSOT4TQAFYCX2JTBIIPWEO7SJO32ZBEYLLQIN,2020-12-31T22:59:41Z
	148,15,GCLNMCOXHGHQQKVEGZG7SB3FO5YVYOPPKTWGMYJ62Z2GXR6VFXA7WBUJ,2020-12-31T22:59:41Z
	148,16,GBZQJYBQU2K5435N5N7KY7BXY5446SK34DMDGWYHLBSLXJQWQRIVJIAF,2020-12-31T22:59:41Z
	148,17,GAIMLETJTXAWYSJ6OCDUET4UGG23ZAJGQDJGDNCSZAQJE4TIHZDE7ITA,2020-12-31T22:59:41Z
	148,18,GAFDT4TODEHRQOBETF4KRZY3R4OTMXIKFV4ZMMZBO2KTVEOWEU4NI5VG,2020-12-31T22:59:41Z
	148,19,GB6DOWJJZ4QJNBD3EPU4SDJILE6FOYRRLV6UJDSN2YYNNQSRADVRQ7UD,2020-12-31T22:59:41Z

Example of the content of `hashlist.txt`:


    e66675e60b946a135b9c30655339e567656e921a4dfd7303191034a702f9d6da
    3efc4a97fb3b7a82fc053cb173cf81bf62658a00d59f0e06fad6efc38225cf53
    c7b82013b804da897be1341ef6930ae4269bf01cadcdc62b7db9a5a3e8dd737f
    912c9def4cec3e204f387912ed00e98a39accbd35aeace73920678f216bd7262
    c2e1cae712b456773a178b73619b54654de70ba9a38854e5c9f52e8b771a3a51
    5a1b9d03a17cc195c71759be7f0cdb95cff275d8dd2faf3fe4959820f662477f
    314b80e908862df91ff31a9ad4e7c5d55f5f73c970ac9fc3bd00821e69a5cef9
    f0533818f449960759a359c6e2e8725efd5b42f24b0f36e89f4bc32f7583901a
    ca99f3c8890802d6743ea3e48e216fe941a16396b3d96f5d746a1e0f11efef7c
    8ea99e3949436b02b570ca3995a9de26e15ef838309e9635c7661b29d40ab21b
    3204114bf95742daf9983e519f44a9b7a138ff834b23904af9565c0e9d2ac750
    884cd6e2d188e9cb3a60b1273573ea7936230582bded26a988ade7e8a9d4ce22
    884a66d2ffe0dc096508312971aa93cbba5d64167074c088eeb3d24b3299d0d9
    6b412324f9706f73207968278e99cfded38808ff77119708ea7aae5e089dbb96
    6a8ffbe944c1a72d9cdaa96766695d43287e34b55922b96dab1b5fbfb388c7ee
    348f8c1c5d07d48d5e7f0da5fdef1921e0375ebae522344bf6c341e68ed4f741

## Usage
On windows:
Put the `walletlookup.exe`, `accounts.csv` and `hashlist.txt` in the same folder and navigate there with PowerShell. You can use the program without one of the file but you will getting a reminder always.

    .\walletlookup.exe -b gcj5l -e swj4

The output will be: 

    Possible wallets:
	Address could be: GCJ5LYIZEVCWESUYSHDHPCPD6LZOY3LJPNCGTDKP5IBYWAXCKSGNSWJ4
	It was created in block 5848222 and it was the 1888521th wallet created.
	Date of creation: 2022-11-23T08:39:34Z
	----------------------------------------------------------------------------

Ofcourse later more wallet can have match to this criteria.
Important to note you can give the data in lower and upper case also.
If you are not setting a mode argument the default the program will looking for wallet.

|argument| result |
|--|--|
| nothing | Looking for wallet |
| -m wallet | Looking for wallet |
| -m account | Looking for wallet |
| -m hash | Looking for transaction hash |
| -m txhash | Looking for transaction hash |

## Functions

    Useful tool to search for Pi wallet address or transaction hash based on partial data

    Usage: walletlookup.exe [OPTIONS]

    Options:
    -m, --mode <MODE>          Depends you are looking for wallet then give wallet or account, if you looking for transaction hash give hash or txhash value [default: wallet]
    -b, --begins <BEGINS>      The exact characters beginning of the wallet, starting with G, please use no more than 28!
    -e, --ends <ENDS>          The exact characters ending of the wallet of the wallet please use no more than 28!
    -c, --contains <CONTAINS>  Characters in the exact order which are in the wallet! Highly optional.
    -h, --help                 Print help
    -V, --version              Print version
