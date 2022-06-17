require_relative './base'

module Operations
  class JR < Base
    def self.template
      ERB.new <<~EOF
        let delta = cpu.read_n() as i8; 

<% if operand2.present? %> if <%= @op1_builder.call %> { <% end %>
        self.pc = ((u32::from(self.reg.pc) as i32) + i32::from(delta)) as u16;
<% if operand2.present? %> } <% end %>
      EOF
    end
  end
end
