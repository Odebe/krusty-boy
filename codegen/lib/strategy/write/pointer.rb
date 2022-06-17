module Strategy
  module Write
    class Pointer
      def self.call(operand, result)
        "cpu.#{operand.clean_name.downcase} = #{result}"
      end
    end
  end
end
