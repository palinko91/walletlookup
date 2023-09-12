## Simple CLI tool to search for exact Pi Wallet Address
In order to use this tool you have to put the `walletlookup` exacutable in the same file with `accounts.csv`.
Since the `accounts.csv` file dynamically expanding and it's huge already not provided to the repo.
You can get it here: [download accounts.csv](https://drive.google.com/file/d/17Acqgf9sPx9x8vmaSyExNphNr2P9LC0Y/view?usp=sharing)
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

## Usage
On windows:
Put the `walletlookup.exe` and `accounts.csv` in the same folder and navigate there with PowerShell.

    .\walletlookup.exe -b gcj5l -e swj4

The output will be: 

    Possible wallets:
	Address could be: GCJ5LYIZEVCWESUYSHDHPCPD6LZOY3LJPNCGTDKP5IBYWAXCKSGNSWJ4
	It was created in block 5848222 and it was the 1888521th wallet created.
	Date of creation: 2022-11-23T08:39:34Z
	----------------------------------------------------------------------------

Ofcourse later more wallet can have match to this criteria.
Important to note you can give the data in lower and upper case also

## Functions

    Useful tool to search for Pi wallet address based on partial data

	Usage: walletlookup.exe [OPTIONS]

	Options:
	  -b, --begins <BEGINS>      The exact characters beginning of the wallet, starting with G, please use no more than 28!
	  -e, --ends <ENDS>          The exact characters ending of the wallet of the wallet please use no more than 28!
	  -c, --contains <CONTAINS>  Characters in the exact order which are in the wallet! Highly optional.
	  -h, --help                 Print help
	  -V, --version              Print version
