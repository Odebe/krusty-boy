module Render
  class Operand
    def initialize(strategy)
      @strategy = strategy
    end

    def call(operand)
      @strategy.call(operand)
    end
  end
end
