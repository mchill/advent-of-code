#!/usr/bin/ruby

begin
    File.open('input.txt', 'r') do |file|
        lights = Array.new(1000) { |light| Array.new(1000) { |light| 0 }}

        file.each_line do |line|
            words = line.split(' ')

            corner1 = words[words.length - 3].split(',')
            corner2 = words[words.length - 1].split(',')

            x1 = corner1[0].to_i
            y1 = corner1[1].to_i
            x2 = corner2[0].to_i
            y2 = corner2[1].to_i

            for row in y1..y2 do
                for col in x1..x2 do
                    if words[0] == 'toggle'
                        lights[row][col] += 2
                    elsif words[1] == 'on'
                        lights[row][col] += 1
                    elsif words[1] == 'off'
                        lights[row][col] = [lights[row][col] - 1, 0].max
                    end
                end
            end
        end

        total = 0
        for row in 0..999 do
            for col in 0...999 do
               total += lights[row][col]
            end
        end

        puts total
    end
rescue SystemCallError
    puts 'input.txt not found.'
end
