require_relative './base'

module Operations
  class INC < Base
    def self.template
      ERB.new <<~EOF
        let value = <%= @op1_builder.call %>;
        
          <%= call %>;
      EOF
    end

    def call
      if operand1.indirect?
        "cpu.write(#{operand1.render_as(::Strategy::Read::Register)}, value + 1)"
      else
        "cpu.registers.set_#{operand1.clean.downcase}(value + 1)"
      end
    end
  end
end
