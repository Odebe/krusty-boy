require_relative 'simple'

module Strategy
  module Read
    class Indirect
      def initialize(base)
        @base = base
      end

      def call(operand)
        "cpu.mmu.read(#{@base.call(operand)})"
      end
    end
  end
end
