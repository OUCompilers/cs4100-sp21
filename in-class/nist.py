import urllib.request, json

with urllib.request.urlopen("https://beacon.nist.gov/beacon/2.0/pulse/last") as url:
    data = json.loads(url.read().decode())

    # Convert hex string to bit string.
    bits = bin(int(data["pulse"]["outputValue"], 16))[2:]
    
    # Add leading zero bits omitted by the conversion.
    bits = "0" * (512 - len(bits)) + bits

    # Split the bits up into pairs.
    pairs = [bits[i:i+2] for i in range(0, len(bits), 2)]

    # Rejection sampler giving 1/3 probability of a quiz.
    for p in pairs:
        if p == "00":
            print("QUIZ!")
            break
        elif p in ["01", "10"]:
            print("NO QUIZ!")
            break
        else:
            print("trying again..")

    # print("done")
