require 'erb'
require 'json'

opcodes = JSON.parse(File.read('opcodes.json'))

template = ERB.new <<~EOF
  match opcode { <% opcodes['unprefixed'].each do |opcode, info| %>
    <%= opcode %> => { <% unless opcode == '0xcb' %>
        println!("<%= info['mnemonic']%> <%= info['operand1'] %> <%= info['operand2']%>");
        self.inc_pc_by(<%= info['length'] %>); <% else %>
        self.inc_pc_by(1);
        let cb_opcode = self.current_opcode();
        match cb_opcode { <% opcodes['cbprefixed'].each do |cb_opcode, cb_info| %>
            <%= cb_opcode %> => {
                println!("<%= cb_info['mnemonic']%> <%= cb_info['operand1'] %> <%= cb_info['operand2']%>");
                self.inc_pc_by(<%= cb_info['length'] -1 %>);
            } <% end %>
            _ => {
              println!("INVALID OPCODE: {:x}", cb_opcode);
              self.inc_pc_by(1);
            }
        } <% end %>
    } <% end %>
    _ => {
      println!("INVALID OPCODE: {:x}", opcode);
      self.inc_pc_by(1);
    }
  }
EOF

puts template.result(binding)
