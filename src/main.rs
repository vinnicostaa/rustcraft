use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_world, setup_player))
        .add_systems(Update, move_camera)
        .run();
}

// Components
// Marker component: não tem dados, só "marca" a entity
// O Bevy usa isso para filtrar nas queries
#[derive(Component)]
struct Player;

// enum = tipo que pode ser UMA de várias variantes
// Perfeito para representar tipos de bloco!
// #derive[(Clone, Copy)] = o Rust gera automáticamente
//      Clone: permite .clone()
//      Copy: permite copiar o valor sem precisar do .clone()
//            (só funciona para tipos simples/pequenos)
#[derive(Clone, Copy)]
enum BlockType {
    Grass,
    Dirt,
    Stone,
}

// Geração de Terreno

// Funcão de "ruido" simples sem lib externa
// Recebe coordenadas x, z e devolve a altura entre 0 e max_height
fn terrain_height(x: i32, z: i32, max_height: i32) -> i32 {
    // Usamos seno e cosseno para criar ondulações no terreno
    // como se fossem colunas suaves
    let fx = x as f32 * 0.3; // as f32 converte i32 para float
    let fz = z as f32 * 0.3;

    let noise = (fx.sin() + fz.cos() + (fx * 0.5).cos()) / 3.0;
    // .sin() e .cos() devolvem valores entre -1.0 e 1.0
    // A média dos três dá uma onda mais interessante

    // Converte de [-1, 1] para [0, max_height]
    let normalized = (noise + 1.0) / 2.0; // agora entre 0.0  e 1.0
    (normalized * max_height as f32) as i32
    // as i32 = trunca o float para inteiro (joga fora a parte decimal)
}

// Decide qual tipo de block vai em cada altura
// match = como um "switch" do Rust, mas muito poderoso
// O compilador OBRIGA você a cobrir todos os campos!
fn block_type_for(y: i32, surface_y: i32) -> BlockType {
    let depth = surface_y - y; // quão fundo estamos da superfície

    match depth {
        0 => BlockType::Grass,    // Superfície = grama
        1..=2 => BlockType::Dirt, // 1 ou 2 blocos abaixo = terra
        // 1..=2 é um RangeInclusive: de 1 até 2, inclusive ambos
        _ => BlockType::Stone, // _ = "qualquer outro valor" = pedra
    }
}

// Setup do mundo
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ResMut = acesso imutavel a um resource global
    // Assets<Mesh> = repositório de malhas 3D gerenciado pelo Bevy

    // Luz direcional (tipo "sol")
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: false, // TODDO: Temporário
            ..default()             // preenche o resto dos valores como padrão.
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Vec3 = vetor 3D (x, y, z) - tipo fundamental em jogos 3D
        // looking_at() rotaciona o Transform para apontar pro ponto dado
    ));

    // Criamos uma malha só — todos os blocos compartilham ela
    // Isso é mais eficiente do que criar uma malha por bloco
    let block_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    // CuboId::new() cria uma malha  de cubo para largura/altura/profundidade
    // meshes.add() salva no repositório e devolve um "handle" (referência)

    // Materiais para cada tipo de bloco
    // Criados UMA vez e reutilizados com .clone() no handle
    let grass_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.1), // verde
        unlit: true,
        ..default()
    });
    let dirt_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.25, 0.1), // marrom terra
        unlit: true,                             // TODO: Temporário
        ..default()
    });
    let stone_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5), // cinza
        unlit: true,                            // TODO: Temporário
        ..default()
    });
    // StandardMaterial = material com iluminação realista (PBR)
    // base_color: a cor base do material - srgb(r, g, b) com valores 0.0 - 1.0

    let chunk_size = 24_i32; // tamanho do "chunk" (área vizivel)
    let max_height = 6_i32; // altura máxima do terreno

    // Gera uma grade de blocos 16x16
    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let surface_y = terrain_height(x, z, max_height);
            // surface_y = altura do topo nessa coluna x,z

            // Gera todos os blocos DA BASE até a superfície (coluna sólida)
            for y in 0..=surface_y {
                // 0..=surface_y inclui surface_y (range inclusivo)

                let block_type = block_type_for(y, surface_y);

                // match para escolher o materia certo
                // note: block_type é Copy, então não precisamos de &
                let material = match block_type {
                    BlockType::Grass => grass_mat.clone(),
                    BlockType::Dirt => dirt_mat.clone(),
                    BlockType::Stone => stone_mat.clone(),
                };
                // O compilador garante que todos os casos são cobertos!
                // Se vocẽ adicionar BlockType::Sand e esquecer aqui -> erro de compilação

                commands.spawn((
                    Mesh3d(block_mesh.clone()),
                    // clone() cria uma cópia do handle - nescessario porque
                    // não podemos "mover" o mesmo handle para varias entidades
                    MeshMaterial3d(material),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                    // as f32 = converte i32 para float de 32bits
                    // Rust não converte tipos automaticamente - você decide!
                ));
            }
        }
    }
}

// Setup do Player
fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_xyz(8.0, 5.0, 20.0).looking_at(
            Vec3::new(8.0, 0.0, 8.0), // ponto que a câmera olha
            Vec3::Y,                  // "cima" é eixo Y
        ),
    ));
    // No Bevy, câmera é só mais uma entity com components!
    // Camera3d diz ao Bevy para renderizar o mundo pela pespectiva dela
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    // Res<ButtonInput<KeyCode>> = Resource de input do teclado (somente leitura)
    // Res<Time> = Resource de info de tempo (delta, elapsed...)
    // &mut Transform = acesso MUTÁVEL ao Transform das entities com Player

    if let Ok(mut transform) = query.single_mut() {
        // single_mut() = pega a única entity que bate com a Query
        // Se tiver 0 ou 2+, vai dar panic! (erro em runtime)
        // Por isso só temos um player no mundo

        let speed = 10.0 * time.delta_secs();
        // delta_secs() = tempo do ultimo frame em segundos
        // Multiplicar pela velocidade = movimento frame-rate indenpendente!
        // Sem isso, o jogo seria mais rápido em PCs mais potentes

        // foward/right são direções RELATIVAS á camera atual
        let forward = transform.forward();
        let right = transform.right();

        if keys.pressed(KeyCode::KeyW) {
            transform.translation += forward * speed;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation -= forward * speed;
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation -= right * speed;
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation += right * speed;
        }
        if keys.pressed(KeyCode::Space) {
            transform.translation += Vec3::Y * speed; // sobe
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            transform.translation -= Vec3::Y * speed; // desce
        }
        // translation = posição da entity no mundo (Vec3)
        // += com Vec3 soma componente a entity (x+x, y+y, z+z)
    };
}
