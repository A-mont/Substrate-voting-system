# Sistema de Votación en Substrate

Este tutorial te guiará a través de los pasos necesarios para crear y usar una pallet de votación básica en Substrate. Cubriremos la configuración del ambiente de desarrollo, la creación de la pallet, y la adición de cada función al runtime.

## Comenzando

Dependiendo de tu sistema operativo y la versión de Rust, podrías necesitar instalar paquetes adicionales para compilar este template. Consulta las instrucciones de [instalación](https://docs.substrate.io/install/) para tu plataforma para identificar las dependencias más comunes.

### Construcción

Usa el siguiente comando para construir el nodo sin lanzarlo:

```sh
cargo build --release
```


### Ejecutar Nodo 

Usa el siguiente comando para ejecutar el nodo:

```sh
./target/release/node-template --dev
```

## Agrega las funciones a tu Pallet

### 1. Almacenamiento de candidatos y sus votos:

Define el almacenamiento para mantener los candidatos y la cuenta de sus votos:

```rust
#[pallet::storage]
	pub type Candidates<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>;
```

### 2. Eventos Personalizados 

Agregamos los eventos perzonalidos que usaremos en la pallet:

```rust
#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		Voted{

			who: T::AccountId,
			candidate: T::AccountId,
		},
		CandidateAdded(T::AccountId),
		CandidateRemoved(T::AccountId)
	}
```

## 3. Errores Personalizados 

Agregamos los errores personalizados que usaremos en la pallet:

```rust
#[pallet::error]
	pub enum Error<T> {
		
		NoneValue,
		StorageOverflow,
	}
```

## 4. Función para Agregar Candidatos

Implementa la función para agregar nuevos candidatos al sistema:

```rust
pub fn add_candidate(origin: OriginFor<T>, candidate_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
	
			// Registrar candidato
			Candidates::<T>::insert(&candidate_id,  0 );
	
			// Emitir evento (opcional)
			Self::deposit_event(Event::CandidateAdded(candidate_id));
	
			Ok(())
		}
```


## 5.  Función para Remover Candidatos:

Implementa la función para eliminar candidatos existentes:

```rust
pub fn remove_candidate(origin: OriginFor<T>, candidate_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			// Eliminar el candidato
			Candidates::<T>::remove(&candidate_id);
		
			// Emitir evento (opcional)
			Self::deposit_event(Event::CandidateRemoved(candidate_id));
		
			Ok(())
		}
```

## 6. Función para Votar por un Candidato

Implementa la función que permite a los usuarios votar por un candidato:

```rust
pub fn vote(origin: OriginFor<T>, candidate: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			// Incrementar el conteo de votos para el candidato
			let result: Result<(), Error<T>> = Candidates::<T>::mutate(candidate.clone(), |vote_count| {
				// Usar `match` para manejar el `Option<u32>`
				*vote_count = match vote_count {
					// Si ya hay algunos votos, intentar sumar 1
					Some(count) => match count.checked_add(1) {
						Some(new_count) => Some(new_count),
						None => return Err(Error::<T>::StorageOverflow.into()), // Manejo adecuado del error
					},
					// Si aún no hay votos, inicializar a 1
					None => Some(1),
				};
				Ok(())
			});
		
			// Verificar resultado de mutate y manejar posible error
			result?;
		
			// Emitir un evento (opcional)
			Self::deposit_event(Event::Voted{who, candidate});
		
			Ok(())
		}
		
```

Felicidades, acabas de personalizar tu primera pallet para agregar al runtime.

## Conclusión

Con este sistema, has creado una pallet de votación básica funcional en Substrate y la has integrado dentro del runtime de tu blockchain. Este ejemplo demuestra la flexibilidad y potencia de Substrate para el desarrollo rápido y seguro de blockchains personalizadas.