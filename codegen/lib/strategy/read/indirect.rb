require_relative 'simple'

module Strategy
  module Read
    class Indirect
      def initialize(base)
        @base = base
      end

      def call(operand)
        "cpu.mmu.read_u8(#{@base.call(operand)})"
      end
    end
  end
end
