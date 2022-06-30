require_relative './base'

module Operations
  class RET < Base
    def self.template
      ERB.new <<~EOF
<% if operand1.present? %> 
        if <%= operand1.render_as(::Strategy::Read::Flag) %> {
          cpu.pc = cpu.stack_pop();
        }
<% else %>
        cpu.pc = cpu.stack_pop();
<% end %>
      EOF
    end
  end
end
