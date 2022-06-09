module Operands
  class Pointer < Generic
    def read_operand
      "cpu.#{clean.downcase}"
    end
  end
end
