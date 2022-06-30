require_relative './base'

module Operations
  class LDH < Base
    def template
      if operand1.addr?
        # LD ($FF00+a8),A
        ERB.new <<~EOF
          let addr = <%= @op1_builder.call %>;
          let value = <%= @op2_builder.call %>;

          <%= write_u8 %>;
        EOF
      else
        # LD A,($FF00+a8)
        ERB.new <<~EOF
          let addr = <%= @op2_builder.call %>;
          self.reg.a = cpu.mmu.read_u8(addr);
        EOF
      end
    end
  end
end
