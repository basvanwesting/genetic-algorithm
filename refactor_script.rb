require 'shellwords'

roots = %w(
  src
  tests
  examples
  benches
)

exclude_dirs = []

underscore_translations = {
  /gene_values/               => 'allele_values',
  /gene_range/                => 'allele_range',
  /with_gene_range/           => 'with_allele_range',
  /with_gene_values/          => 'with_allele_values',
  /gene_value_index_sampler/  => 'allele_value_index_sampler',
  /gene_value_index_samplers/ => 'allele_value_index_samplers',
  /gene_value_sampler/        => 'allele_value_sampler',
  /gene_value_samplers/       => 'allele_value_samplers',
  /gene_value_size/           => 'allele_value_size',
  /gene_value_sizes/          => 'allele_value_sizes',
  /gene_multi_range/          => 'allele_multi_range',
  /gene_multi_values/         => 'allele_multi_values',
  /with_gene_multi_range/     => 'with_allele_multi_range',
  /with_gene_multi_values/    => 'with_allele_multi_values',
}

other_translations = {
  /BinaryGene/          => 'BinaryAllele',
  /ContinuousGene/      => 'ContinuousAllele',
  /DefaultDiscreteGene/ => 'DefaultDiscreteAllele',
  /DiscreteGene/        => 'DiscreteAllele',
}

#check:
#rg -i -o --no-line-number --no-filename '\w*context\w*' | sort | uniq -c

refactor_code = true
refactor_filenames = true

translations = underscore_translations.merge(other_translations)
translations.each do |from, to|
  roots.each do |dirname|
    filenames = Dir.glob(File.join(dirname, '**', '*'))
    filenames.each do |filename|
      p filename
      next if File.directory?(filename)
      next if exclude_dirs.detect { |exclude_dir| filename[exclude_dir] }
      body = File.read(filename)
      new_body = body.gsub(from, to)
      if body != new_body
        puts "replace #{from} to #{to} in #{filename}"
        File.write(filename, new_body) if refactor_code
      end
    end
  end
end

underscore_translations.each do |from, to|
  roots.each do |dirname|
    filenames = Dir.glob(File.join(dirname, '**', '*'))
    filenames.each do |filename|
      next if File.directory?(filename)
      next if exclude_dirs.detect { |exclude_dir| filename[exclude_dir] }
      next unless filename =~ from
      new_filename = filename.gsub(from,to)
      new_directory = File.dirname(new_filename)
      mkdir_p_command = "mkdir -p #{Shellwords.escape(new_directory)}"
      puts mkdir_p_command
      %x[#{mkdir_p_command}] if refactor_filenames
      mv_command = "mv #{Shellwords.escape(filename)} #{Shellwords.escape(new_filename)}"
      puts mv_command
      %x[#{mv_command}] if refactor_filenames
    end
  end
end



