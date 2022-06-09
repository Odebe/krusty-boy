require 'erb'
require 'json'
require 'byebug'

require_relative 'lib/case'

opcodes = JSON.parse(File.read('opcodes.json'))
opcodes = opcodes['unprefixed']

# opcodes = opcodes.select { |_k, v| v['mnemonic'] == 'LD' }
#
# opcodes.each do |k, v|
#   puts Case.new(k, v).build
# end

code = '0x11'
opcode_case = Case.new(code, opcodes[code])

text = opcode_case.build
puts text

# fname = './meta3.rs'

# File.delete(fname)
# f = File.open(fname, 'w')
# f.write(text)
# f.close
