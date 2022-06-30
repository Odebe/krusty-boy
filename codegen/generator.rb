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

    0xcb => {
      let postfix = cpu.read_opcode();

      match postfix {
        <% prefixed.each do |rcase| %>
          <%= rcase.build %>
        <% end %>
      }
    }
  }
}
EOF

cases = []
prefixed = []

opcodes['unprefixed'].each do |k, v|
  cases << Case.new(k, v)
end

opcodes['cbprefixed'].each do |k, v|
  prefixed << Case.new(k, v)
end

text = template.result(binding)

fname = './meta3.rs'

File.delete(fname)
f = File.open(fname, 'w')
f.write(text)
f.close

`rustfmt #{fname}`
