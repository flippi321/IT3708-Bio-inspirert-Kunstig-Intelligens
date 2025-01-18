using Random
using Colors
using Plots
using ProgressMeter

function generate_functions()
    #=Library of all the possible functions the binary
    can activate. Returns the library and its size.=#
    library = [
        (x, y) -> sin(x) + cos(y),
        (x, y) -> sin(x) * cos(y),
        (x, y) -> -sin(x) * cos(y),
        (x, y) -> -cos(x) * sin(y),
        (x, y) -> cos(x + y),
        (x, y) -> tan(x - y),
        (x, y) -> sin(x^2 + y^2),
        (x, y) -> cos(x * y),
        (x, y) -> x^2 - y^2,
        (x, y) -> x^3 + y^3,
        (x, y) -> x^4 - 2*x^2 * y^2 + y^4,
        (x, y) -> (x^2 + y^2)^(1/2),
        (x, y) -> (x^2 + y^2) * (x + y),
        (x, y) -> exp(-x^2 - y^2),
        (x, y) -> exp(x + y),
        (x, y) -> log(abs(x^2 + y^2 + 1)),
        (x, y) -> exp(x * y),
        (x, y) -> log(abs(x) + abs(y) + 1),
        (x, y) -> (x + y) / (1 + x^2 + y^2),
        (x, y) -> (x^2 - y^2) / (1 + x^2 + y^2),
        (x, y) -> abs(x - y),
        (x, y) -> abs(x^2 + y^2),
        (x, y) -> sin(x + y) * cos(x - y),
        (x, y) -> sqrt(x^2 + y^2),
        (x, y) -> x * log(abs(y + 1)),
        (x, y) -> (x^2 + y^2)^0.5 * cos(x * y),
        (x, y) -> tanh(x * y),
        (x, y) -> sin(x + y) * exp(x * y),
        (x, y) -> log(abs(x - y) + 1) + sin(x * y),
        (x, y) -> exp(x) * cos(y) + x^2 - y^2
    ]
    return library, length(library)
end


function periodic_boundary(x, y, domain_min, domain_max)
    #=Periodic boundary which stops the particles from leaving the domain.=#
    x = (x .- domain_min) .% (domain_max - domain_min) .+ domain_min
    y = (y .- domain_min) .% (domain_max - domain_min) .+ domain_min
    return x, y
end


function binary_to_rgb(binary_array::Vector{Int64})
    #=Converts binary code to RGB values.=#
    red = parse(Int, join(binary_array[1:8]), base=2)
    green = parse(Int, join(binary_array[9:16]), base=2)
    blue = parse(Int, join(binary_array[17:24]), base=2)

    return (red/255, green/255, blue/255)
end


function encode_particle_properties(binary_values, library)
    #=Creates a function which applies functions from the
        function library based on binary values provided.=#
    selected_funcs = library[binary_values .== 1]
    return (x, y) -> sum(f(x, y) for f in selected_funcs)
end

function binary_string_to_vector(binary_string::String)
    #=Converts a binary string into a vector of integers.=#
    return [parse(Int, c) for c in binary_string]
end


function generate_particles(n, bounds, rng)
    #=Generates random initial states for each particle.=#
    xs = collect(range(bounds[1], bounds[2], length=n))
    ys = collect(range(bounds[1], bounds[2], length=n))
    shuffle!(rng, ys) #The RNG seed is used to get deterministic results
    return [(x, y) for (x, y) in zip(xs, ys)]
end

function generate_random_individual(particle_groups, library_size)
    #=Generates a random individual.=#

    individual = ""

    for i in 1:particle_groups
        color = rand([0, 1], 24)
        x_velocity = rand([0, 1], library_size)
        #At least one function needs to be activated to get some kind of velocity
        x_velocity = all(x -> x == 0, x_velocity) ? (x_velocity[rand(1:length(x_velocity))] = 1; x_velocity) : x_velocity
        y_velocity = rand([0, 1], library_size)
        #At least one function needs to be activated to get some kind of velocity
        y_velocity = all(x -> x == 0, y_velocity) ? (y_velocity[rand(1:length(x_velocity))] = 1; y_velocity) : y_velocity
        particle_group = vcat(color, x_velocity, y_velocity)
        individual *= join(particle_group, "")
    end
        
    return individual
end

function generate_taylor_green_vortex(particle_groups, library_size)
    #=Generates a bitstring which decodes as the 2D Taylor-Green Vortex.=#

    individual = ""

    for i in 1:particle_groups
        color = rand([0, 1], 24)
        x_velocity = zeros(Int, library_size)
        x_velocity[2] = 1
        y_velocity = zeros(Int, library_size)
        y_velocity[4] = 1
        particle_group = vcat(color, x_velocity, y_velocity)
        individual *= join(particle_group, "")
    end
        
    return individual
end

function decode_individual(individual, particle_groups, library, library_size, particles_per_individual, bounds)
    #=Decodes the bitstring into colors and velocity functions 
    which the animation function will need.
        Also deterministically creates an initial state
        for the particles in all the particle groups.=#

    colors = []
    x_velocities = []
    y_velocities = []
    particles = []

    for i in 1:particle_groups
        group_end = 24 + library_size * 2
        start_idx = Int(1 + (i - 1) * group_end)
        end_idx = Int(group_end + (i - 1) * group_end)
        current_part = individual[start_idx:end_idx]
        current_part = binary_string_to_vector(current_part)

        color = binary_to_rgb(current_part[1:24])
        colors = vcat(colors, color)

        velocity_func_x = encode_particle_properties(current_part[25:25+(library_size-1)], library)
        x_velocities = vcat(x_velocities, velocity_func_x)
        velocity_func_y = encode_particle_properties(current_part[25+library_size:group_end], library)
        y_velocities = vcat(y_velocities, velocity_func_y)

        particle_group = generate_particles(particles_per_individual, bounds, MersenneTwister(Int(i)))
        particles = push!(particles, collect(particle_group))

    end

    return (colors, x_velocities, y_velocities, particles)
    
end

function animate_particles(decoded_individual, steps, duration, bounds)
    #=Creates an animation based on the decoded bitstring.=#

    colors = decoded_individual[1]
    x_velocities = decoded_individual[2]
    y_velocities = decoded_individual[3]
    particle_groups = decoded_individual[4]

    p = Progress(steps) #Progress meter initialization

    Δt = duration / steps
    anim = @animate for step in 1:steps
        scatter() #Refreshes the frame at the start of each step
        for i in eachindex(colors) #This is done for each particle group in each animation step
            #Adds color:
            cur_color = RGB(colors[i][1], colors[i][2], colors[i][3])
            #Calculates velocities:
            velocities = [(x_velocities[i](x, y), y_velocities[i](x, y)) for (x, y) in particle_groups[i]]
            #Applies velocities:
            particle_groups[i] = [(x + Δt * vx, y + Δt * vy) for ((x, y), (vx, vy)) in zip(particle_groups[i], velocities)]
            #Applies boundary conditions:
            particle_groups[i] .= [periodic_boundary(x, y, bounds[1], bounds[2]) for (x, y) in particle_groups[i]]
            
            #Draws the particle at their new position
            scatter!([p[1] for p in particle_groups[i]], [p[2] for p in particle_groups[i]];
                color=cur_color, label="", xlim=(bounds[1], bounds[2]), ylim=(bounds[1], bounds[2]), markersize=2) 
        end
        next!(p) #Progress meter update
    end

    gif(anim, "particle_animation.gif", fps=30)
    
end

function main()

    #Function library
    library, library_size = generate_functions()

    #Particle settings
    particles_per_individual = 1000
    total_particles = 10000
    particle_groups = total_particles/particles_per_individual
    bounds = (0, 2*π)
    
    #Animation settings
    steps = 90
    duration = 1.5

    #Create a bitstring
    #individual = generate_random_individual(particle_groups, library_size)
    individual = generate_taylor_green_vortex(particle_groups, library_size)

    #Decode bitstring
    decoded_individual = decode_individual(individual, particle_groups, library, library_size, particles_per_individual, bounds)

    #Animate decoded bitstring
    animate_particles(decoded_individual, steps, duration, bounds)
end

main()