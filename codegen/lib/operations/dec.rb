require_relative './base'

module Operations
  class DEC < Base
    def self.template
      ERB.new <<~EOF
<% if operand1.indirect? %>
       cpu.write_u8(<%= operand1.render_as(::Strategy::Read::Register) %>, <%= call %>);
<% else %>
      cpu.reg.<%= operand1.clean.downcase %> = <%= call %>;
<% end %>
      EOF
    end

    def call
      "cpu.alu_dec(#{@op1_builder.call})"
    end
  end
end
