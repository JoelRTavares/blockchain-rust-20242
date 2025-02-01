#![cfg_attr(not(feature = "std"), no_std, no_main)]

//use ink_lang as ink;
#[ink::contract]
mod flipper {
    use ink::prelude::string::String;
    use scale_info::prelude::format;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};
    //use serde::{Serialize, Deserialize};

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Genero {
        Acao,
        Animacao,
        Comedia,
        Drama,
        Gospel,
        Suspense,
        Outros,
    }
    
    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Filme {
        nome: String,
        bilhetes_vendidos: u32,
        ano_lancamento: u16, 
        mes_lancamento: u8,
        dia_lancamento: u8,
        genero: Genero,
    }

    #[ink(storage)]
    pub struct Flipper {
        lista_filmes: Vec<Filme>,
        lista_nomes: Vec<String>,
    }

    impl Flipper {
        /// Construtor que inicializa um filme de exemplo.
        #[ink(constructor)]
        pub fn new_with_example() -> Self {
            let filme_exemplo = Filme {
                nome: String::from("Filme Exemplo"),
                bilhetes_vendidos: 1000,
                ano_lancamento: 2025,
                mes_lancamento: 1,
                dia_lancamento: 1,
                genero: Genero::Acao,
            };
            Self {
                lista_nomes: Vec::from([filme_exemplo.nome.clone()]),
                lista_filmes: Vec::from([filme_exemplo]),
            }
        }
        #[ink(constructor)]
        pub fn new_with_custom(
            nome: String,
            bilhetes_vendidos: u32,
            ano_lancamento: u16,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero) -> Self {
            let filme_exemplo = Filme {
                nome,
                bilhetes_vendidos,
                ano_lancamento,
                mes_lancamento,
                dia_lancamento,
                genero,
            };
            Self {
                lista_nomes: Vec::from([filme_exemplo.nome.clone()]),
                lista_filmes: Vec::from([filme_exemplo]),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default()  -> Self {
            Self {
                lista_filmes: Vec::new(),
                lista_nomes: Vec::new(),
            }
        }

        /// A message that can be called on instantiated contracts.
        #[ink(message)]
        pub fn add_filme(
            &mut self,
            nome: String,
            bilhetes_vendidos: u32,
            ano_lancamento: u16,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero,
        ) -> Result<(), String> {
            let novo_filme = Filme {
                nome,
                bilhetes_vendidos,
                ano_lancamento,
                mes_lancamento,
                dia_lancamento,
                genero,
            };

            if let Err(e) = self.checa_data(ano_lancamento, mes_lancamento, dia_lancamento) { 
                return Err(e);
            }

            if  self.checa_nome_unico(&novo_filme.nome){
                return Err( "Esse nome já existe no sistema!".to_string(),);
            }

            self.lista_nomes.push(novo_filme.nome.clone());
            self.lista_filmes.push(novo_filme);
          
            Ok(())
        }

        #[ink(message)]
        pub fn get_lista_filmes(&self) -> Vec<Filme> {
            self.lista_filmes.clone()
        }
        #[ink(message)]
        pub fn delete_filme(
            &mut self,
            nome: String,
        ) -> Result<(), String> {

            let ind = match self.get_index_filme(&nome) {
                Ok(num) => num,
                Err(e) => {
                    return Err(e);
                },
            };
          
            self.lista_nomes.remove(ind);
            self.lista_filmes.remove(ind);
            Ok(())
        }
        #[ink(message)]
        pub fn update_filme(
            &mut self,
            nome_filme_a_atualizar:String,
            novo_nome_filme:String,
            bilhetes_vendidos: u32,
            ano_lancamento: u16,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero,
        ) -> Result<(), String> {
            let ind = match self.get_index_filme(&nome_filme_a_atualizar) {
                Ok(num) => num,
                Err(e) => {
                    return Err(e);
                },
            };

            if novo_nome_filme != "" {
                if  self.checa_nome_unico(&novo_nome_filme){
                    return Err( "Esse nome já existe no sistema!".to_string(),);
                }
                else{
                    self.lista_nomes[ind] = novo_nome_filme.clone();
                    self.lista_filmes[ind].nome = novo_nome_filme;
                }
            }
            if bilhetes_vendidos != 0 {
                self.lista_filmes[ind].bilhetes_vendidos = bilhetes_vendidos;
            }

            if ano_lancamento != 0 || mes_lancamento != 0 || dia_lancamento != 0 {
                if let Err(e) = self.checa_data(ano_lancamento, mes_lancamento, dia_lancamento) { 
                    return Err(e);
                }
                else{
                    self.lista_filmes[ind].ano_lancamento = ano_lancamento;
                    self.lista_filmes[ind].mes_lancamento = mes_lancamento;
                    self.lista_filmes[ind].dia_lancamento = dia_lancamento;
                }
            }
            self.lista_filmes[ind].genero = genero;
            Ok(())
        }


        //Validadores
        pub fn get_index_filme(&self, nome_f: &str) -> Result<usize, String>{
            if self.lista_nomes.len() == 0 {
                return Err(String::from("Não existem filmes a serem deletados!"));
            }

            for (index, filme) in self.lista_nomes.iter().enumerate(){
                if filme == nome_f {
                    return Ok(index);
                }
            }
            return Err(format!("Não existe um filme com esse nome!\nFilmes disponíveis: {:#?}", &self.lista_nomes));
        }

        pub fn checa_nome_unico(&self, nome_f: &str) -> bool{
            if self.lista_nomes.len() == 0 {
                return false;
            }

            for i in &self.lista_nomes{
                if i == nome_f {
                    return true;
                }
            }
            return false
        }

        pub fn checa_data(&self, ano: u16, mes: u8, dia: u8) ->Result<(), String> {
            if ano < 2000 || ano > 2025 {
                return Err(String::from("Por favor, insira um ano válido (Entre 2000 e 2025)!"));
            }
            if mes < 1 || mes > 12 {
                return Err(String::from("Por favor, insira um mês válido (Entre 1 e 12)!"));
            }
            if dia < 1 || dia > 31 {
                 return Err(String::from("Por favor, insira um dia válido (Entre 1 e 31)!"));
            }

            match mes {
                4 | 6 | 9 | 11 if dia > 30 => return Err(String::from("Data inválida!")),
                2 => {
                    if ano % 4 == 0 && (ano % 100 != 0 || ano % 400 == 0) {
                        if dia > 29 {
                            return Err(String::from("Data inválida!"))
                        }
                    } else if dia > 28{
                       return Err(String::from("Data inválida!")) 
                    }
                }
                _ => return Ok(()),
            }
            return Ok(())
        }
    }
   
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get_lista_filmes().is_empty(), true);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn with_example_works() {
            let flipper = Flipper::new_with_example();
            let filme_exemplo = &flipper.get_lista_filmes()[0];

            assert_eq!(filme_exemplo.nome, "Filme Exemplo");
            assert_eq!(filme_exemplo.bilhetes_vendidos, 1000);
            assert_eq!(filme_exemplo.ano_lancamento, 2025);
            assert_eq!(filme_exemplo.mes_lancamento, 1);
            assert_eq!(filme_exemplo.dia_lancamento, 1);
            assert_eq!(filme_exemplo.genero, Genero::Acao);
        }
        #[ink::test]
        fn with_custom_works() {
            let nome_f = String::from("Novo filme");
            let bilhetes_v = 200;
            let ano_l = 2005;
            let mes_l = 10;
            let dia_l = 2;
            let gen = Genero::Acao;

            let flipper = Flipper::new_with_custom(nome_f.clone(), bilhetes_v, ano_l, mes_l, dia_l, gen.clone());
            let filme_exemplo = &flipper.get_lista_filmes()[0];

            assert_eq!(filme_exemplo.nome, nome_f);
            assert_eq!(filme_exemplo.bilhetes_vendidos, bilhetes_v);
            assert_eq!(filme_exemplo.ano_lancamento, ano_l);
            assert_eq!(filme_exemplo.mes_lancamento, mes_l);
            assert_eq!(filme_exemplo.dia_lancamento, dia_l);
            assert_eq!(filme_exemplo.genero, gen);
        }

    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::default();

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value().is_empty(), true));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn example_movie_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::new_with_example();

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
    
            assert_eq!(filmes.len(), 1, "A lista de filmes deveria conter exatamente um filme!");

            let filme_exemplo = &filmes[0];

            assert_eq!(filme_exemplo.nome, "Filme Exemplo");
            assert_eq!(filme_exemplo.bilhetes_vendidos, 1000);
            assert_eq!(filme_exemplo.ano_lancamento, 2025);
            assert_eq!(filme_exemplo.mes_lancamento, 1);
            assert_eq!(filme_exemplo.dia_lancamento, 1);
            assert_eq!(filme_exemplo.genero, Genero::Acao);

            Ok(())
        }

        /*
        #[ink_e2e::test]
        async fn example_movie_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::new(false);
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
        */
    }
}
