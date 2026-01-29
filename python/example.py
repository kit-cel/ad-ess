import numpy as np

from pyadess import AdEss

num_data_bits = 15
sequence_length = 10
distribution = [ # Probabilities for the amplitudes 1, 3, 5, 7
    0.1, # P(1)
    0.2, # P(3)
    0.3, # P(5)
    0.4  # P(7)
]
res_factor = 5 # Resolution factor for calculating the weights from the distribution

adess = AdEss.new_for_distribution_num_bits(
    num_data_bits,
    sequence_length,
    distribution,
    res_factor
)

print()
print(f'Created a new AdESS instance which maps {adess.num_data_bits()} bits to {sequence_length} amplitudes.')
print(f'Shaping rate: {num_data_bits/sequence_length:.2f} bit/amplitude')
print(f'Average sequence energy: {adess.average_energy()}')
print(f'AD-ESS amplitude distribution: {np.round(adess.amplitude_distribution(), 2)}')
print()

rng = np.random.default_rng(0)


print('# Testing encoding / decoding')

tx_bits = rng.integers(0, 1, size=num_data_bits, endpoint=True)
print('Sent bits:', tx_bits)

tx_sequence = adess.encode(tx_bits)
print('Encoded into amplitudes:', tx_sequence)

rx_sequence = tx_sequence # no noise

rx_bits = adess.decode(rx_sequence)
print('Decoded into bits:', rx_bits)

if np.all(rx_bits == tx_bits):
    print('Encode / decode successfull!')
else:
    # This should never happen as the channel has no noise
    print('Encode / decode failure!')

print()
print('# Testing bulk encoding / decoding')

num_transmissions = 1000

tx_bits = rng.integers(0, 1, size=(num_transmissions, num_data_bits), endpoint=True)

tx_sequence = adess.multi_encode(tx_bits)

rx_sequence = tx_sequence # no noise

rx_bits = adess.multi_decode(rx_sequence)

if np.all(rx_bits == tx_bits):
    print(f'Encode / decode of {num_transmissions} transmissions successfull!')
else:
    # This should never happen as the channel has no noise
    print('Encode / decode failure!')
