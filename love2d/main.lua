objects = {}

function love.load()
	love.window.setMode(1200,700, {vsync = 0})
	bush = love.graphics.newImage("bush.png")
	math.randomseed( os.time() )
	for i = 1, 50000, 1 do
		local object = {x=math.random(0, 1200), y=math.random(0,600), direction=0}
		table.insert(objects, object)
	end
end

function love.draw()
    love.graphics.print("Current FPS: ".. tostring(love.timer.getFPS()), 0, 0)
    for _, value in pairs(objects) do
    	love.graphics.draw(bush, value.x, value.y)
    end
end

function love.update(dt)
	for _, value in pairs(objects) do
		if value.direction == 0 then
			value.y = value.y + 150 * dt
		else
			value.y = value.y - 150 * dt
		end
		if value.y > 600 then
			value.direction = 1

			else if value.y < 0 then
				value.direction = 0
			end
		end
	end
end
