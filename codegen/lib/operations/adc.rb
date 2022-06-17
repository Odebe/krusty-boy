require_relative './base'

module Operations
  class ADC < Base
    def self.template
      ERB.new <<~EOF
        cpu.reg.a = <%= call %>;
      EOF
    end

    def call
      "cpu.alu_adc(#{@op1_builder.call}, #{@op2_builder.call})"
    end
  end
end
