require_relative 'operand_builder'

Dir[File.dirname(__FILE__) + '/operations/*.rb'].each do |file|
  require_relative file
end

class Case
  def self.template
    ERB.new  <<~EOF
      <%= opcode %> => { 
        println!("<%= mnemonic %>");

        <%= operation.build if operation %>

        <%= info['cycles'].first %>
      },
    EOF
  end

  attr_reader :info

  def initialize(opcode, info)
    @opcode = opcode
    @info = info
  end

  def opcode
    @opcode
  end

  def name
    info['mnemonic']
  end

  def mnemonic
    "#{ info['mnemonic'] } #{ info['operand1'] } #{ info['operand2'] }"
  end

  def operand1
    OperandBuilder.new('operand1', info['operand1'], only: info['operand2'].nil?)
  end

  def operand2
    OperandBuilder.new('operand2', info['operand2'])
  end

  def operation
    Operations.const_get(info['mnemonic']).new(operand1, operand2)
  rescue NameError
    nil
  end

  def build
    self.class.template.result(binding)
  end
end
