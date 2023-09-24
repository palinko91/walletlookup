# GOOGLE COLAB
# In the other section run
# !pip install stellar-sdk
from stellar_sdk import Server,StrKey
from stellar_sdk.xdr import TransactionEnvelope
import csv

server = Server(horizon_url="https://api.mainnet.minepi.com/")

################################################################
# ONLY EDIT HERE
################################################################
# The latest block mined on mainnet you can check https://blockexplorer.minepi.com/blocks
latest_block = 10809164
# Should be +1 greater than the last processed block
start_processing_from_block = 8103579
###############################################################

total_account_number = 0
with open('./drive/MyDrive/org_accounts.csv', 'r') as file:
    lines = list(csv.reader(file))
    total_account_number = int(lines[-1][1])

for i in range(start_processing_from_block,latest_block+1):
  if i>1: #minimal block height is 2
    block_height = str(i)
    print(f"Block height: {block_height}")
    print(f"Total account number: {total_account_number}")
    block_raw = server.transactions().for_ledger(i).call()
    records = block_raw['_embedded']['records']
    records_length = len(records)
    for num in range(records_length):
      if bool(records[num]['successful'])==True:
        timestamp = records[num]['created_at']
        hash = records[num]['hash']
        with open('/content/drive/MyDrive/org_hashlist.txt', 'a') as hash_file:
          writer_hash = hash_file.write
          writer_hash(hash + '\n')
        envelope_xdr = TransactionEnvelope.from_xdr(records[num]['envelope_xdr'])
        envelope_len = len(envelope_xdr.v1.tx.operations)
        for num in range(envelope_len):
          trtype = envelope_xdr.v1.tx.operations[num].body.type
          if str(trtype) == "OperationType.CREATE_ACCOUNT":
            public_key = envelope_xdr.v1.tx.operations[num].body.create_account_op.destination.account_id.ed25519.uint256
            wallet_address = StrKey.encode_ed25519_public_key(public_key)
            total_account_number += 1
            print(f"New total account number: {total_account_number}")
            file = open('./drive/MyDrive/org_accounts.csv', 'a')
            writer = csv.writer(file)
            data = [block_height,total_account_number,wallet_address,timestamp]
            writer.writerow(data)
            file.close()
  print("Block data saved")

print("DONE")