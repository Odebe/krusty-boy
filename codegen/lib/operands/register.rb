module Operands
  class Registers < Generic
    def read_operand
      value = "cpu.registers.#{clean.downcase}"
      value << '()' if clean.chars.count > 1
      value
    end
  end
end
