require_relative '../checkers'

module Operands
  class Generic
    include Checkers

    def initialize(key, mnem)
      @key = key
      @mnem = mnem
    end

    def build
      read
    end

    def read
      if indirect? && @key == 'operand2'
        read_indirect
      else
        read_operand
      end
    end

    def read_indirect
      "cpu.mmu.read(#{read_operand})"
    end

    def read_operand
      "cpu.read_#{source_name.downcase}()"
    end
  end
end
