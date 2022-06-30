require_relative './base'

module Operations
  class CALL < Base
    def self.template
      ERB.new <<~EOF
<% if operand2.present? %>
        let value = <%= @op2_builder.call %>;
    
        if <%= operand1.render_as(::Strategy::Read::Flag) %> {
            cpu.stack_push(cpu.pc);
            cpu.pc = value;            
        }
<% else %>
        let value = <%= @op1_builder.call %>;

        cpu.stack_push(cpu.pc);
        cpu.pc = value;   
<% end %>
      EOF
    end
  end
end
