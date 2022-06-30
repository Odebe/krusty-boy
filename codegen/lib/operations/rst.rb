require_relative './base'

module Operations
  class RST < Base
    def self.template
      ERB.new <<~EOF
        cpu.stack_push(cpu.pc);
        cpu.pc = <%= @op1_builder.call %>_u16;
      EOF
    end
  end
end
