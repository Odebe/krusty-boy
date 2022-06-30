require_relative './base'

module Operations
  class JP < Base
    def self.template
      ERB.new <<~EOF
<% if operand2.present? %>
        let value = <%= @op2_builder.call %>;

        if <%= operand1.render_as(::Strategy::Read::Flag) %> {
            cpu.pc = value;
        }
<% else %>
        cpu.pc = <%= @op1_builder.call %>;
<% end %>
      EOF
    end
  end
end
