require 'erb'
require 'json'
require 'byebug'

require_relative 'lib/case'

opcodes = JSON.parse(File.read('opcodes.json'))

template = ERB.new  <<~EOF
use crate::cpu::Cpu;

pub fn exec_opcode(cpu: &mut Cpu) -> u8 {
  let opcode = cpu.read_opcode();

  match opcode {
    <% cases.each do |rcase| %>
      <%= rcase.build %>
    <% end %>
  }
EOF


# opcodes = opcodes['unprefixed']
opcodes = opcodes['cbprefixed']

# opcodes = opcodes.select { |_k, v| %w[SWAP].include? v['mnemonic'] }

all_count = opcodes.keys.count
generated = 0

todo = []
done =[]

cases = []

opcodes.each do |k, v|
  c = Case.new(k, v)
  if c.operation.nil?
    todo << c.name
  else
    done << c.name
    generated += 1
  end

  cases << c
end

text = template.result(binding)


# puts "Реализовано: #{generated}/#{all_count}"
# puts "Done: #{done.uniq.inspect}"
# puts "TODO: #{todo.uniq.inspect}"

# code = '0xc6'
# opcode_case = Case.new(code, opcodes[code])
#
# text = opcode_case.build
# puts text


fname = './meta3.rs'

File.delete(fname)
f = File.open(fname, 'w')
f.write(text)
f.close
