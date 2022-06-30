require_relative './base'

module Operations
  class RETI < Base
    def self.template
      ERB.new <<~EOF
        cpu.pc = cpu.stack_pop();
        cpu.ei = true;
      EOF
    end
  end
end
