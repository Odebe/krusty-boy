require_relative './base'

module Operations
  class BIT < Base
    def self.template
      ERB.new <<~EOF
      <% if operand2.indirect? %>
        let addr = <%= operand2.render_as(::Strategy::Read::Register) %>;
        let op1 = cpu.mmu.read_u8(addr);
      <% else %>
        let op1 = <%= @op2_builder.call %>;
      <% end %>

        <%= add_func_call %>;
      EOF
    end

    def add_func_call
      "cpu.alu_bit(op, #{@op1_builder.call})"
    end
  end
end
